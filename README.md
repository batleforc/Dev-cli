# Dev-cli

The goal of this project is to provide a cli to hasten my dev process and to create interface for thing that actualy dont.

## Eclipse Che

Eclipse che is my main env that allow me to dev in my personal time (and i love it).

### What do i need ?

- Start/Restart/stop a workspaces from the CLI (Understand the lifecycle of a workspaces)
- Restart the workspaces from the local devfile (A FEATURE THAT I REALLY NEED)
- Get my current workspaces (if in one or else choose one)
- List other container (in the current workspace or a choosen one) and if there is get the hability to open a shell inside
- Quick connect my vscode to the workspaces (code --folder-uri "vscode-remote://k8s-container+context={CONTEXT NAME}+podname={Pod Name}+namespace={Namespace}+name={Container name}+image={Container image}{Path to open in the project}")
- Quick connect Idea (jetbrains gateway) if handled by the workspaces


## Dockerfile templating

Long story short i would like to include in another submodule/cli close to this one (it's possible that in the futur i pop out this part) a module that would act has a component library who would output a Dockerfile including all your need. Atm i have made the batleforc/che-base that include the base tool that i deemed necessary and made some variant based on it including different tool (sdkman for a full java image, Rust and cargo for a rust variant, etc). BUT i may need in some case to reduce the size of those image (ATM less than 1Go).

## Choice to be made

- Rust or Golang ?
- Lib mode ? 


## What i want to lean

- Interact directly with the kubernetes api not throught the mainstream manifest
- Know a lots more of my ide (VsCode) and whats really behind
- Deep dive in my dev env (Eclipse Che)
- Create a CI/CD that dont create Docker images but binary for different env (Do i publish the release on github or on a FS ?)