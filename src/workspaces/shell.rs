use crate::{
    config::CurrentWorkspace,
    shell::{find_pod_by_ws_name::find_pod_by_ws_name, select_pod},
};
use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use k8s_openapi::api::core::v1::Pod;
use kube::api::AttachParams;
use kube::api::TerminalSize;
use tracing::event;

#[cfg(unix)]
use tokio::signal;
use tokio::{io::AsyncWriteExt, select};

#[cfg(unix)]
// Send the new terminal size to channel when it change
async fn handle_terminal_size(mut channel: Sender<TerminalSize>) -> Result<(), anyhow::Error> {
    let (width, height) = crossterm::terminal::size()?;
    channel.send(TerminalSize { height, width }).await?;

    // create a stream to catch SIGWINCH signal
    let mut sig = signal::unix::signal(signal::unix::SignalKind::window_change())?;
    loop {
        if (sig.recv().await).is_none() {
            return Ok(());
        }

        let (width, height) = crossterm::terminal::size()?;
        channel.send(TerminalSize { height, width }).await?;
    }
}

#[cfg(windows)]
// We don't support window for terminal size change, we only send the initial size
async fn handle_terminal_size(mut channel: Sender<TerminalSize>) -> Result<(), anyhow::Error> {
    let (width, height) = crossterm::terminal::size()?;
    channel.send(TerminalSize { height, width }).await?;
    let mut ctrl_c = tokio::signal::windows::ctrl_c()?;
    ctrl_c.recv().await;
    Ok(())
}

#[tracing::instrument(level = "trace")]
pub async fn spawn_shell(
    current_workspace: CurrentWorkspace,
    container_name: Option<String>,
    shell: String,
) -> anyhow::Result<()> {
    let client = match current_workspace.get_client().await {
        Some(iencli) => iencli,
        None => return Ok(()),
    };
    let pod = match find_pod_by_ws_name(client.clone(), current_workspace.clone()).await {
        Some(pod) => pod,
        None => {
            event!(tracing::Level::ERROR, "Pod's not found");
            return Ok(());
        }
    };
    let container_cible = match select_pod::select_pod(container_name, pod.clone()) {
        Some(c_name) => c_name,
        None => return Ok(()),
    };
    let pod_api = current_workspace.get_api::<Pod>(client);

    let mut attach_param = AttachParams::interactive_tty();
    attach_param.container = Some(container_cible);

    let mut process = match pod_api
        .exec(&pod.metadata.name.unwrap(), vec![shell], &attach_param)
        .await
    {
        Ok(attached_process) => {
            event!(tracing::Level::TRACE, "Success createing remote process");
            attached_process
        }
        Err(err) => {
            event!(
                tracing::Level::ERROR,
                ?err,
                "Error while creating remote process"
            );
            return Ok(());
        }
    };

    // The following is taken from https://github.com/kube-rs/kube/blob/main/examples/pod_shell_crossterm.rs
    // Even if it's taken 
    crossterm::terminal::enable_raw_mode()?;
    let mut stdin = tokio_util::io::ReaderStream::new(tokio::io::stdin());
    let mut stdout = tokio::io::stdout();

    let mut output = tokio_util::io::ReaderStream::new(process.stdout().unwrap());
    let mut input = process.stdin().unwrap();

    let term_tx = process.terminal_size().unwrap();

    let mut handle_terminal_size_handle = tokio::spawn(handle_terminal_size(term_tx));

    loop {
        select! {
            message = stdin.next() => {
                match message {
                    Some(Ok(message)) => {
                        input.write_all(&message).await?;
                    }
                    _ => {
                        break;
                    },
                }
            },
            message = output.next() => {
                match message {
                    Some(Ok(message)) => {
                        stdout.write_all(&message).await?;
                        stdout.flush().await?;
                    },
                    _ => {
                        break
                    },
                }
            },
            result = &mut handle_terminal_size_handle => {
                match result {
                    Ok(_) => println!("End of terminal size stream"),
                    Err(e) => println!("Error getting terminal size: {e:?}")
                }
            },
        };
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
