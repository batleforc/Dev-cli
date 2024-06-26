# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.5.1](https://github.com/batleforc/dev-cli/compare/f111e9c933a01bd656c90a6e336556ec3af7baa9..0.5.1) - 2024-04-06
#### Bug Fixes
- follow good practice by clippy - ([1979a53](https://github.com/batleforc/dev-cli/commit/1979a538784365fa2dfb65aeeddd08838fb42cfc)) - Max
#### Features
- update readme - ([f111e9c](https://github.com/batleforc/dev-cli/commit/f111e9c933a01bd656c90a6e336556ec3af7baa9)) - Max

- - -

## [0.5.0](https://github.com/batleforc/dev-cli/compare/f5f4d2cfc23e002ca75fa2a7c5fa3fe5ed404594..0.5.0) - 2024-04-06
#### Bug Fixes
- select what can be short or not (in the arg way) - ([da69711](https://github.com/batleforc/dev-cli/commit/da6971193baff476130e69649cac71a98ea020be)) - max
#### Features
- Setup of the healthcheck workflow ++ rework every tracing to follow the tracing::{kind}!(message) workflow - ([8e3e424](https://github.com/batleforc/dev-cli/commit/8e3e424faeee8d49eb71c6387454ab59d6d66ae5)) - Max
- add more dock - ([c2abaa1](https://github.com/batleforc/dev-cli/commit/c2abaa173aa5e51eaf9cdc65d5a91bc15e7e7264)) - max
- Create open_vs_code and ask_if_pod_should_up - ([4d44aec](https://github.com/batleforc/dev-cli/commit/4d44aecf59d34086ba45d678f0d43bfbcc9a52f6)) - Max
- Extract subfonction - ([d916820](https://github.com/batleforc/dev-cli/commit/d916820b06ed3649a190f31b8a34364c7a6a91f8)) - Max
- start creating the cmd(and go work on the zorin pc) - ([5af5428](https://github.com/batleforc/dev-cli/commit/5af542869eee4ec81b02e27500763af83c8458ba)) - max
- select what's needed for the openvscode cmd ++ add fix for healthcheck ++ define that if a command is not provided redirect to help - ([ba004de](https://github.com/batleforc/dev-cli/commit/ba004de7d6cecff032368f843a3bc88d47fad55a)) - max
- Add a little doc on the activity service used by the che-code extension - ([ba74db1](https://github.com/batleforc/dev-cli/commit/ba74db1a7ebd4113d0add7a3f324552cf1368ad0)) - Max
- Setup vscode/open_code in the sandbox env (and it work cog) - ([3086add](https://github.com/batleforc/dev-cli/commit/3086addbb0fa32cf9323136e82bf2516deadfaa5)) - Max
- move to rustsec/audit-check@v1.4.1 - ([6efddf8](https://github.com/batleforc/dev-cli/commit/6efddf8dc662f972addd30e742b3520429dc195d)) - max
- mise en place audit journalier - ([df208af](https://github.com/batleforc/dev-cli/commit/df208afe07238b0d08a00ae6aa6a6a209e95709c)) - max
- Start opencode struct - ([0494bcb](https://github.com/batleforc/dev-cli/commit/0494bcbb439377c285d51e64dc0135acfe798dc1)) - max
- split spawn_shell cmd - ([55b05c1](https://github.com/batleforc/dev-cli/commit/55b05c1c4cfdd8d6a96953cf43e5b2540e9ba497)) - max
- explain what's done and what's left to do - ([ae3a44a](https://github.com/batleforc/dev-cli/commit/ae3a44afe73021a2251f1023ae2be88291b4dff2)) - max
- add more info - ([f5f4d2c](https://github.com/batleforc/dev-cli/commit/f5f4d2cfc23e002ca75fa2a7c5fa3fe5ed404594)) - max

- - -

## [0.4.0](https://github.com/batleforc/dev-cli/compare/02b9bb0b7974df236c355d06c9e19e8b30264545..0.4.0) - 2024-03-26
#### Features
- move from full handled to crossterm in order to have a better handle of the terminal workflow (K9s workkkkk) - ([31fa657](https://github.com/batleforc/dev-cli/commit/31fa65783c0ae837e96830de1b5724bd1d3a129f)) - max
- Mise en place du shell a travers du tokio stdin de base - ([ee832fb](https://github.com/batleforc/dev-cli/commit/ee832fb0a99665d6805e92cb4623b2192b065c47)) - max
- prepare process - ([a36c6ed](https://github.com/batleforc/dev-cli/commit/a36c6edb84a562a0acad7d72bd0f2595cfffbeb5)) - max
- Mise en place selecteur de containeur pour la commande shell - ([4545a9f](https://github.com/batleforc/dev-cli/commit/4545a9ff89017c45bf3b02c766eeb67985942868)) - max
- factorize list container - ([ea6d497](https://github.com/batleforc/dev-cli/commit/ea6d4974f5a609b6e2a7f24ab120dfd970666214)) - max
- get pod from devworkspace name - ([0de7464](https://github.com/batleforc/dev-cli/commit/0de746400f5a72177e95fe993cfd8761b5a0fb6a)) - max
- start shell spec - ([35944da](https://github.com/batleforc/dev-cli/commit/35944da32c29c53cb3307e0e93ee08e711a25eef)) - max
- add extension wdhongtw.gpg-indicator - ([162a5d6](https://github.com/batleforc/dev-cli/commit/162a5d6ecb90683540154a4b281115b104afe788)) - Maxime
- include an arg that will wait for the restart to be finished - ([0da5383](https://github.com/batleforc/dev-cli/commit/0da53836b72bad1a6db786b64bf4e5f30edb34de)) - max
- setup restart - ([02b9bb0](https://github.com/batleforc/dev-cli/commit/02b9bb0b7974df236c355d06c9e19e8b30264545)) - max

- - -

## [0.3.1](https://github.com/batleforc/dev-cli/compare/589eb93c0ee108171ff31a311e546c66ba7ec29e..0.3.1) - 2024-03-18
#### Bug Fixes
- resolve code scanning alert - ([589eb93](https://github.com/batleforc/dev-cli/commit/589eb93c0ee108171ff31a311e546c66ba7ec29e)) - max

- - -

## [0.3.0](https://github.com/batleforc/dev-cli/compare/f6d1e97329b8ec3ca09b0551bef3b940040d9a64..0.3.0) - 2024-03-18
#### Features
- Start/Stop ws ++ basic lifecycle - ([9323330](https://github.com/batleforc/dev-cli/commit/9323330d2223d44583d9d8d8265a0c41a8eae42f)) - max
- WIP: devworkspaces Start/Stop - ([9248adc](https://github.com/batleforc/dev-cli/commit/9248adc9c37bad62bcb9b2f0fdd86b9ebb261fb9)) - max
- facto code to follow the same client/api instance - ([4637dad](https://github.com/batleforc/dev-cli/commit/4637dad8f1634186545c05b9a4fef8d508ada962)) - max
- Explain the versioning process - ([f6d1e97](https://github.com/batleforc/dev-cli/commit/f6d1e97329b8ec3ca09b0551bef3b940040d9a64)) - max

- - -

## [0.2.2](https://github.com/batleforc/dev-cli/compare/0.2.1..0.2.2) - 2024-03-17
#### Features
- work on version increment in cargo toml - ([d355155](https://github.com/batleforc/dev-cli/commit/d35515568b43505a9f4943860a40fb7965a4ebaa)) - max
- work on version increment in cargo toml - ([24fd4d7](https://github.com/batleforc/dev-cli/commit/24fd4d7cf959d166ff2b87d45a973039f3c60f67)) - max
- disable windows and linux for futur version - ([9e12271](https://github.com/batleforc/dev-cli/commit/9e12271a8c8be2497bcd52d0ed0e0af143e2ad11)) - max

- - -


## [0.2.1](https://github.com/batleforc/dev-cli/compare/0.2.0..0.2.1) - 2024-03-17

#### Features

- change regex - ([1909b95](https://github.com/batleforc/dev-cli/commit/1909b95a1b36df2ac2da3029a69a9cbd2e99525a)) - max

- - -

## [0.2.0](https://github.com/batleforc/dev-cli/compare/0.1.0..0.2.0) - 2024-03-17

#### Features

- change cog config - ([13e3fe8](https://github.com/batleforc/dev-cli/commit/13e3fe8c2b5b2f4b34543df07b289bf90161046f)) - max

- - -

## [0.1.0](https://github.com/batleforc/dev-cli/compare/4f597e0e1620dca6ea1b649077cb03e9e49faefa..0.1.0) - 2024-03-17

#### Bug Fixes

- found a tag working ? - ([3b4ca3e](https://github.com/batleforc/dev-cli/commit/3b4ca3edb37834c9d23b38ecd76766e38603be5d)) - max
- try to fix regex - ([8e90196](https://github.com/batleforc/dev-cli/commit/8e9019674696de99924aa4624c20039da808c3ae)) - max
- dev de la fatigue - ([f2d6ba8](https://github.com/batleforc/dev-cli/commit/f2d6ba83ca835f1bcb9d6f0c4ed1981f53855351)) - max

#### Features

- work on bump - ([037bf85](https://github.com/batleforc/dev-cli/commit/037bf85eeb34a6bbb8f106937ddaecf85e38ebe4)) - max
- setup build-release - ([7c1f386](https://github.com/batleforc/dev-cli/commit/7c1f386b090194b39d20e1031903e053361b21c8)) - max
- start work on new CICD - ([2661ce1](https://github.com/batleforc/dev-cli/commit/2661ce1ef4ab2c8bc3f3f8f3517e488c5f2eef95)) - max
- add more config and fix security - ([0cfc43a](https://github.com/batleforc/dev-cli/commit/0cfc43a306eb9fa7d75715f6208d37c09ced0110)) - max
- update action - ([1dde4ee](https://github.com/batleforc/dev-cli/commit/1dde4ee1e0e89cbb446ef77ba3d6398ca9c3533a)) - Maxime
- update codeql - ([26ea04e](https://github.com/batleforc/dev-cli/commit/26ea04eb92e2f30ca6d82f04744ad739c9aa8246)) - Maxime
- explain what's needed in the fetch - ([605b83a](https://github.com/batleforc/dev-cli/commit/605b83a8b91e34a46380423f4025147d132895de)) - max
- report advancement - ([0467bf9](https://github.com/batleforc/dev-cli/commit/0467bf9d7872a9aed8e9500fca78721a59011765)) - max
- add a litle bit of doc about the validator - ([37117ff](https://github.com/batleforc/dev-cli/commit/37117ff277e096d5fec7a559ae3f67231ccd1ed7)) - max
- Mise en place validator for devfile - ([ae845e3](https://github.com/batleforc/dev-cli/commit/ae845e3bbb3874253e95b6a55ed0f7a17177c1d8)) - max
- remove duplicate extensions - ([8ab9e88](https://github.com/batleforc/dev-cli/commit/8ab9e88ca29e1b039207398dea3ce2d7a8ba0d9a)) - max
- Mise en place include_str for art - ([ad662ee](https://github.com/batleforc/dev-cli/commit/ad662ee37172990aa6b344fd23c5f957c98644c8)) - max
- explain the feature of the Devfile Handling - ([57945f8](https://github.com/batleforc/dev-cli/commit/57945f81f55b7b36cbb4cf995c3cf9a6a4b49765)) - max
- correct display - ([8775b26](https://github.com/batleforc/dev-cli/commit/8775b268d3eff6f0c0ece1a838f665b57151b821)) - max
- mise en place de la command list workspace qui detecte aussi si on est dans le workspace - ([8a36008](https://github.com/batleforc/dev-cli/commit/8a36008745d58ce52771f666c35ba2705d3e6dd8)) - max
- Mise en place generation de CRD - ([68a7b32](https://github.com/batleforc/dev-cli/commit/68a7b3211192995e7d766deb249cd778d50a37e7)) - max
- more data - ([34f64e1](https://github.com/batleforc/dev-cli/commit/34f64e1074a96c844c7b31b1b23c1225303f55de)) - max
- add how the restart from localdevfile work - ([9857b87](https://github.com/batleforc/dev-cli/commit/9857b878eb6fa895449c438d1f4ea72862822123)) - max
- pass down the workspace - ([7a5ee51](https://github.com/batleforc/dev-cli/commit/7a5ee516c93e7344a62a4d03bfee430a749d3423)) - max
- mise en place de la fonction get pods - ([c18eff5](https://github.com/batleforc/dev-cli/commit/c18eff525055e29e3d10cbb07eb371dc5e6129cf)) - [@batleforc](https://github.com/batleforc)
- a little clean up - ([dc30c60](https://github.com/batleforc/dev-cli/commit/dc30c606f32aadfffedd369f01948bcafaef14ae)) - max
- add ascii art ++ format trace ++ first connection to the kuve api - ([c34f495](https://github.com/batleforc/dev-cli/commit/c34f49553726b52e5c3d2d8ce1872eb33a2bf14e)) - max
- remove old file - ([c5564ca](https://github.com/batleforc/dev-cli/commit/c5564ca7eaac5acd7f5657fb35d4eea927dd5f38)) - max
- append to the log in order to not lose previous one - ([87ea248](https://github.com/batleforc/dev-cli/commit/87ea248137864ad4db9cf4da81e33320b534ae48)) - max
- mise en place du tracing pour comprendre vraiment pourquoi ca crash - ([6651061](https://github.com/batleforc/dev-cli/commit/66510610f069b776bad0711c0db16a360f4ccd04)) - max
- init tracing ++ basics info for get (not finished) - ([22149b4](https://github.com/batleforc/dev-cli/commit/22149b4f0ed3b5e55a5c815a52d9acc1c029bbb5)) - max
- add doc about the shell spawn - ([20dfef4](https://github.com/batleforc/dev-cli/commit/20dfef4f3e210640759fb5e7de4867de436509fb)) - max
- more info on what i want to learn - ([8ebde51](https://github.com/batleforc/dev-cli/commit/8ebde514ce043848e44fb1cb2ae0292260a6c063)) - max
- Extract command to a sub module ++ init verbose with enum ++ init a possible async workflow - ([142f181](https://github.com/batleforc/dev-cli/commit/142f1815b751c431aa3865d559ffbf1b911c746c)) - max
- init interface cli - ([16bd13a](https://github.com/batleforc/dev-cli/commit/16bd13a85bf4ac3aa8bab7fb665e38ad738ff57a)) - max
- init project really and then include more info - ([26f7c47](https://github.com/batleforc/dev-cli/commit/26f7c47929bcbd92a57c96824c8f291348c8ed40)) - max
- more doc - ([568dd0f](https://github.com/batleforc/dev-cli/commit/568dd0fc4012ff224b53407cc3a21fac49084992)) - max
- rust has been the chosen one - ([36328b1](https://github.com/batleforc/dev-cli/commit/36328b166359d9509901f1070615af80a0836fa9)) - max
- Add devfile and write some specs - ([9685e47](https://github.com/batleforc/dev-cli/commit/9685e475ae6038141e23b89f027c267a69f26adb)) - max
- init project - ([ef1f0d9](https://github.com/batleforc/dev-cli/commit/ef1f0d98c29446a13c047abd6ccc8dd894662284)) - max

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).
