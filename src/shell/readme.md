# Shell functionality

The goal is double, executing one time command and spawning an interactive terminal

## Hot to identify a pod that bellongs to a devfile

LABELS ==> controller.devfile.io/devworkspace_name="{DEVFILE NAME}"

## Func needed

- [x] Find DevWorkspace's pod name in:(client, devworkspace's name) out:(pod's name)
- [ ] run an interactive command in a container in:(command,pod and container name) out:(nothing)
- [ ] Run a one time command in a contianer in:(command,pod and container name) out:(noting)
