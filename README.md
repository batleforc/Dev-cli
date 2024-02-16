# Dev-cli

The goal of this project is to provide a cli to hasten my dev process and to create interface for thing that actualy dont.

## Eclipse Che

Eclipse che is my main env that allow me to dev in my personal time (and i love it). This project is born from [this issue](https://github.com/eclipse/che/issues/22812) that follow my thinking process of loving project with source openly available.

### What do i need ?

- Start/Restart/stop a workspaces from the CLI (Understand the lifecycle of a workspaces)
- Restart the workspaces from the local devfile (A FEATURE THAT I REALLY NEED)
- Get my current workspaces (if in one or else choose one)
- List other container (in the current workspace or a choosen one) and if there is get the hability to open a shell inside
- Quick connect my vscode to the workspaces (code --folder-uri "vscode-remote://k8s-container+context={CONTEXT NAME}+podname={Pod Name}+namespace={Namespace}+name={Container name}+image={Container image}{Path to open in the project}")
- Quick connect Idea (jetbrains gateway) if handled by the workspaces

### NotaBene

- To detect DevSpace/Eclipse Che there is some env var that's available.
![exemple](./doc/image/var-env-in-devspaces.png)
- You can start/stop with a simple patch [Code Source](https://github.com/che-incubator/che-code/blob/6e0a908d58cacb380c216dde3af544d75e3913d5/code/extensions/che-api/src/impl/k8s-workspace-service-impl.ts#L62)

## Dockerfile/DevFile templating

Long story short i would like to include in another submodule/cli close to this one (it's possible that in the futur i pop out this part) a module that would act has a component library who would output a Dockerfile including all your need. Atm i have made the batleforc/che-base that include the base tool that i deemed necessary and made some variant based on it including different tool (sdkman for a full java image, Rust and cargo for a rust variant, etc). BUT i may need in some case to reduce the size of those image (ATM less than 1Go).

##  Choice to be made

- Rust or Golang ? (Most likely to be in rust)
- Lib mode ?

### Why rust ?

Rust is my main language to learn atm. And i have lots of project that im currently doing and some capacity that this project bring will further improve those project.

The choosen main lib is [clap - tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html) for the cli part and [kube](https://docs.rs/kube/latest/kube/index.html) for the kubernetes part.

## What i want to learn

- Interact directly with the kubernetes api not throught the mainstream manifest
- Know a lots more of my ide (VsCode) and whats really behind
- Deep dive in my dev env (Eclipse Che)
- Create a CI/CD that dont create Docker images but binary for different env (Do i publish the release on github or on a FS ?)
- Sign commit and a fully fledged Changelog
