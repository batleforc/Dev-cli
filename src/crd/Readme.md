# How to generate the needed CRD

1. Install [Kopium](https://github.com/kube-rs/kopium) with `cargo install kopium`
2. Generate the CRD with `kopium {CRD name}.{CRD group} -A -D PartialEq > {Crd name in snake case}.rs`
3. Enjoy

## CRD to watch and generate

- [devworkspaces.workspace.devfile.io](https://github.com/devfile/api/blob/main/schemas/latest/dev-workspace.json) => dev_work_space.rs
