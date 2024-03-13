# Devfile Handling

Any devfile related stuff will be handled here.

## Restart from needed

Well has we are in the process in setting up (rewriting) the restart from [local|url] feature we need to understand what they made and how it works

### Useful link

- <https://github.com/che-incubator/che-code/blob/main/code/extensions/che-remote/src/extension.ts#L75> The code is where the command is setup in che-code.
- <https://github.com/che-incubator/che-code/blob/main/code/extensions/che-api/src/impl/k8s-devfile-service-impl.ts#L73> This is the place where the devfile is updated
- <https://github.com/eclipse-che/che-devfile-registry/blob/main/tools/devworkspace-generator/src/main.ts#L32> this code handle the generation of the first step of generating your new devfile
- <https://github.com/eclipse-che/che-devfile-registry/blob/main/tools/devworkspace-generator/src/generate.ts#L37> And here another midlle step that will call the context generator the contente will be generated line 65
- <https://github.com/eclipse-che/che-devfile-registry/blob/main/tools/devworkspace-generator/src/devfile-schema/devfile-schema-validator.ts> This module handle the DevFile Validator (split into devfile version thing that i will need to do)

### What we need

- A [validator](https://docs.rs/jsonschema/latest/jsonschema/) that will make sure that our devfile follow the intended format. (The lib need to be tested)
- A fetch that will handle recuperating the content of the devfile
- An updater that will update the devfile inside the devspaces (that is present in the CRD)
