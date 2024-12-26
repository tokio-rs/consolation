# Changelog

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.14 - (2024-12-26)


### Added

- Add the WatchState API (#582) ([7c1f9f2](https://github.com/tokio-rs/console/commit/7c1f9f216f499a0309ecf597c721252186e72c82))
- Improve error msg when state streaming API is unimplemented (#598) ([6ef148a](https://github.com/tokio-rs/console/commit/6ef148a33fe71a682338bd65dd9271fd043086f2))


## 0.1.13 - (2024-10-24)


### Added

- Add large future lints ([#587](https://github.com/tokio-rs/console/pull/587)) ([ae17230](https://github.com/tokio-rs/console/commit/ae1723091fcc76597e78bae39129a48d8cd515c9))

### Fixed

- Correct the grammar issue ([#579](https://github.com/tokio-rs/console/pull/579)) ([f8e1bee](https://github.com/tokio-rs/console/commit/f8e1bee760358f702ca8359ec3de6cb39649fe60))


## 0.1.12 - (2024-07-29)

### Fixed

- Handle Windows path correctly ([#555](https://github.com/tokio-rs/console/pull/555)) ([6ad0def](https://github.com/tokio-rs/console/commit/6ad0def9c4ac3d4e85ad8b7247ca270ff07b45b8))
- Avoid crash when accessing selected item ([#570](https://github.com/tokio-rs/console/pull/570)) ([9205e15](https://github.com/tokio-rs/console/commit/9205e1594b960dbdd3ad4189c996aa1be09e76a8))

### Updated

- Upgrade tonic to 0.12 ([#571](https://github.com/tokio-rs/console/pull/571)) ([5f6faa2](https://github.com/tokio-rs/console/commit/5f6faa22d944735c2b8c312cac03b35a4ab228ef))


## tokio-console-v0.1.11 - (2024-06-10)

### Added

- Replace target column with kind column in tasks view ([#478](https://github.com/tokio-rs/console/pull/478)) ([903d9fa](https://github.com/tokio-rs/console/commit/903d9fa9f9d2dddec2235206b792c264ed9892fb))
- Add flags and configurations for warnings ([#493](https://github.com/tokio-rs/console/pull/493)) ([174883f](https://github.com/tokio-rs/console/commit/174883f15747fe8069bcfd0750c0c07b3acaa232))
- Add `--allow` flag ([#513](https://github.com/tokio-rs/console/pull/513)) ([8da7037](https://github.com/tokio-rs/console/commit/8da70370340401ebc7e82076780d5186a939118c))

### Documented

- Add note about running on Windows ([#510](https://github.com/tokio-rs/console/pull/510)) ([a0d20fd](https://github.com/tokio-rs/console/commit/a0d20fd62df07470b6033524afc00a96d156aaa5))

### Fixed

- Ignore key release events ([#468](https://github.com/tokio-rs/console/pull/468)) ([715713a](https://github.com/tokio-rs/console/commit/715713abda2f2ac22e84f7cf286fed9d723d22f7))
- Accept only `file://`, `http://`, `https://` URI ([#486](https://github.com/tokio-rs/console/pull/486)) ([031bddd](https://github.com/tokio-rs/console/commit/031bdddd2b0828e8407f09bdb8f0be473bd72bc1))
- Fix column sorting in resources tab ([#491](https://github.com/tokio-rs/console/pull/491)) ([96c65bd](https://github.com/tokio-rs/console/commit/96c65bd739444f450e9236c7d9e55d384238d6cb))
- Only trigger lints on async tasks ([#517](https://github.com/tokio-rs/console/pull/517)) ([4593222](https://github.com/tokio-rs/console/commit/45932229fb5aea7a4994a7644bded9baf2776ea8))
- Remove duplicate controls from async ops view ([#519](https://github.com/tokio-rs/console/pull/519)) ([f28ba4a](https://github.com/tokio-rs/console/commit/f28ba4abcf1644b10d260797806f7425b391b226))
- Add pretty format for 'last woken' time ([#529](https://github.com/tokio-rs/console/pull/529)) ([ea11ad8](https://github.com/tokio-rs/console/commit/ea11ad8d6040ef564952b80d58abc713376b6160))


## tokio-console-v0.1.10 - (2023-09-29)

[c8c4a85](https://github.com/tokio-rs/console/commit/c8c4a85df2da55c9745df6f38e19631e84ed0cf5)...[c8c4a85](https://github.com/tokio-rs/console/commit/c8c4a85df2da55c9745df6f38e19631e84ed0cf5)

### <a id = "tokio-console-v0.1.10-breaking"></a>Breaking Changes
- **Update Tonic and Prost dependencies ([#364](https://github.com/tokio-rs/console/issues/364))** ([f9b8e03](https://github.com/tokio-rs/console/commit/f9b8e03bd7ee1d0edb441c94a93a350d5b06ed3b))<br />This commit updates the public dependencies `prost` and `tonic` to
semver-incompatible versions (v0.11.0 and v0.8.0, respectively). This is
a breaking change for users who are integrating the `console-api` protos
with their own `tonic` servers or clients.
- **Update `tonic` to v0.10 and increase MSRV to 1.64 ([#464](https://github.com/tokio-rs/console/issues/464))** ([96e62c8](https://github.com/tokio-rs/console/commit/96e62c83ef959569bb062dc8fee98fa2b2461e8d))<br />This is a breaking change for users of `console-api` and
`console-subscriber`, as it changes the public `tonic` dependency to a
semver-incompatible version. This breaks compatibility with `tonic`
0.9.x and `prost` 0.11.x.

### Added

- [**breaking**](#tokio-console-v0.1.10-breaking) Update Tonic and Prost dependencies ([#364](https://github.com/tokio-rs/console/issues/364)) ([f9b8e03](https://github.com/tokio-rs/console/commit/f9b8e03bd7ee1d0edb441c94a93a350d5b06ed3b))
- Only suggest opening issues for panics ([#365](https://github.com/tokio-rs/console/issues/365)) ([da2a89c](https://github.com/tokio-rs/console/commit/da2a89c0481277be78034e3c60c978d7a7a0f59b))
- Init error handling before subcmds ([#365](https://github.com/tokio-rs/console/issues/365)) ([ec66eda](https://github.com/tokio-rs/console/commit/ec66eda67fd3a4de626299bd979cb174a2dd87bd))
- Filter out boring frames in backtraces ([#365](https://github.com/tokio-rs/console/issues/365)) ([95a5e54](https://github.com/tokio-rs/console/commit/95a5e54269451848e2e0ca2c6108f279c6297127))
- Include config options in autogenerated issues ([#365](https://github.com/tokio-rs/console/issues/365)) ([3244a1f](https://github.com/tokio-rs/console/commit/3244a1f7f8958d76ba8bfa5afaf9da551f100414))
- Reduce decimal digits in UI ([#402](https://github.com/tokio-rs/console/issues/402)) ([c13085e](https://github.com/tokio-rs/console/commit/c13085e381b71177f4b1a05be7c628a04e3b6991))
- Use tokio task ids in task views ([#403](https://github.com/tokio-rs/console/issues/403)) ([f5b06d2](https://github.com/tokio-rs/console/commit/f5b06d2854c0a638aac7ce48a7a2eeaef615e9b9))
- Add support for Unix domain sockets ([#388](https://github.com/tokio-rs/console/issues/388)) ([a944dbc](https://github.com/tokio-rs/console/commit/a944dbcff2de49e45d5fa99edb227c85a5c3d40f), closes [#296](https://github.com/tokio-rs/console/issues/296))
- Add scheduled time per task ([#406](https://github.com/tokio-rs/console/issues/406)) ([f280df9](https://github.com/tokio-rs/console/commit/f280df94100d24e868ce3f9fbfec160677d8a124))
- Add task scheduled times histogram ([#409](https://github.com/tokio-rs/console/issues/409)) ([d92a399](https://github.com/tokio-rs/console/commit/d92a39994f6e759ddba4e53ab7263a0c4edb0b67))
- Update `tonic` to 0.9 ([#420](https://github.com/tokio-rs/console/issues/420)) ([48af1ee](https://github.com/tokio-rs/console/commit/48af1eef6352bd35c607267d68b24cf16033beeb))
- Update MSRV to Rust 1.60.0 ([b18ee47](https://github.com/tokio-rs/console/commit/b18ee473aa499aa581117baea7404623d98b081c))
- Migrate to `ratatui` and update `crossterm` ([#425](https://github.com/tokio-rs/console/issues/425)) ([b209dd6](https://github.com/tokio-rs/console/commit/b209dd654b4929870aa8856e61b9b4f41bbe6f5b))
- Help view modal ([#432](https://github.com/tokio-rs/console/issues/432)) ([359a4e7](https://github.com/tokio-rs/console/commit/359a4e7fa72911e47e2f9daa1e1a04ecaf84afbc))
- Add way to inspect details of task from resource view ([#449](https://github.com/tokio-rs/console/issues/449)) ([132ed4e](https://github.com/tokio-rs/console/commit/132ed4e9db58b4ef3b4a4e42c3dd825bc0d9e532), closes [#448](https://github.com/tokio-rs/console/issues/448))
- Add warning for tasks that never yield ([#439](https://github.com/tokio-rs/console/issues/439)) ([d05fa9e](https://github.com/tokio-rs/console/commit/d05fa9ee6456dd9a9eec72c5299c32a4f0c845c0))
- [**breaking**](#tokio-console-v0.1.10-breaking) Update `tonic` to v0.10 and increase MSRV to 1.64 ([#464](https://github.com/tokio-rs/console/issues/464)) ([96e62c8](https://github.com/tokio-rs/console/commit/96e62c83ef959569bb062dc8fee98fa2b2461e8d))

### Documented

- Update screenshots in README ([#419](https://github.com/tokio-rs/console/issues/419)) ([e9bcd67](https://github.com/tokio-rs/console/commit/e9bcd67aea4469c576a199367858b5e02909fb95))
- Revert "update screenshots in README ([#419](https://github.com/tokio-rs/console/issues/419))" ([993a3d9](https://github.com/tokio-rs/console/commit/993a3d9786fdfdb042211bcb7079e4fb71cdefef))
- Update screenshots in README ([#421](https://github.com/tokio-rs/console/issues/421)) ([8a27f96](https://github.com/tokio-rs/console/commit/8a27f963101f3f0131229a93c1757647421477af))
- Add column descriptions for all tables ([#431](https://github.com/tokio-rs/console/issues/431)) ([e3cf82b](https://github.com/tokio-rs/console/commit/e3cf82b189d23724c72b2437195d0d4a0e421c1d))
- Update MSRV version docs to 1.64 ([#467](https://github.com/tokio-rs/console/issues/467)) ([94a5a51](https://github.com/tokio-rs/console/commit/94a5a5117b85e723c28fafa1eadabf31057570c3))

### Fixed

- Fix ascii-only flipped input ([#377](https://github.com/tokio-rs/console/issues/377)) ([652ac34](https://github.com/tokio-rs/console/commit/652ac3442325aafc8d57e7f1949db51bde010004))
- Declare `tokio-console` bin as `default-run` ([#379](https://github.com/tokio-rs/console/issues/379)) ([9ce60ec](https://github.com/tokio-rs/console/commit/9ce60ecbe2a195ccbd1ee8e1cb573ebadb125061))
- Make `retain_for` default to 6s if not specfied ([#383](https://github.com/tokio-rs/console/issues/383)) ([0a6012b](https://github.com/tokio-rs/console/commit/0a6012b57b39673e4f9a5c95c34d648f857e55fe), fixes [#382](https://github.com/tokio-rs/console/issues/382))
- Enable view-switching keystrokes on details views ([#387](https://github.com/tokio-rs/console/issues/387)) ([f417d7a](https://github.com/tokio-rs/console/commit/f417d7a32561d91a355a38b200e4c8481d3c93da))
- Fix `ViewOptions` default lang' ([#394](https://github.com/tokio-rs/console/issues/394)) ([a1cf1b8](https://github.com/tokio-rs/console/commit/a1cf1b813ac9c02e8c6180dea8f5200dbe5fcc09), fixes [#393](https://github.com/tokio-rs/console/issues/393))
- Remove `tracing-subscriber` 0.2 from dependencies ([#404](https://github.com/tokio-rs/console/issues/404)) ([768534a](https://github.com/tokio-rs/console/commit/768534a44b46c291b5ccaf63e6615c9d528180fe))
- Fix calculation of busy time during poll ([#405](https://github.com/tokio-rs/console/issues/405)) ([e2c536a](https://github.com/tokio-rs/console/commit/e2c536abc536828da032d5d378e526243ac363dd))
- Remove histogram minimum count ([#424](https://github.com/tokio-rs/console/issues/424)) ([02cf8a6](https://github.com/tokio-rs/console/commit/02cf8a6fb653c8da92f44046bad8c8451f8e1ce8))
- Remove trailing space from task/resource location ([#443](https://github.com/tokio-rs/console/issues/443)) ([90e5918](https://github.com/tokio-rs/console/commit/90e591887414e9a1435a709454486c340b50cd44))
- Make long locations readable ([#441](https://github.com/tokio-rs/console/issues/441)) ([9428d7f](https://github.com/tokio-rs/console/commit/9428d7fc99351e22f5d3b756c506d1792d74de9a), closes [#411](https://github.com/tokio-rs/console/issues/411))
- Fix task detail view Id to display remote tokio::task::Id ([#455](https://github.com/tokio-rs/console/issues/455)) ([70c3952](https://github.com/tokio-rs/console/commit/70c3952571cf9fa2523cd3a02a6c3edc28140f55))

## tokio-console-v0.1.9 - (2023-07-03)

[5900300](https://github.com/tokio-rs/console/commit/59003004a6f2f2857be267061f23d34e2257e0f0)...[daa3d51](https://github.com/tokio-rs/console/commit/daa3d51895b52c11e1fd216becbc37b083e9758f)


### Added

- Help view modal ([#432](https://github.com/tokio-rs/console/issues/432)) ([5156e8e](https://github.com/tokio-rs/console/commit/5156e8e951521d1858929d4f52addda5f1a43941))

### Documented

- Add column descriptions for all tables ([#431](https://github.com/tokio-rs/console/issues/431)) ([2de5b68](https://github.com/tokio-rs/console/commit/2de5b68d1a00a77d03a4817f955f385e494368bd))

### Fixed

- Remove histogram minimum count ([#424](https://github.com/tokio-rs/console/issues/424)) ([2617504](https://github.com/tokio-rs/console/commit/26175044cca81cb4a8289841a0c3b458f2d287f1))
- Remove trailing space from task/resource location ([#443](https://github.com/tokio-rs/console/issues/443)) ([29a09ad](https://github.com/tokio-rs/console/commit/29a09adf07eeb56be6233a7333d4cdee4fa954a2))
- Make long locations readable ([#441](https://github.com/tokio-rs/console/issues/441)) ([daa3d51](https://github.com/tokio-rs/console/commit/daa3d51895b52c11e1fd216becbc37b083e9758f), closes [#411](https://github.com/tokio-rs/console/issues/411))

## tokio-console-v0.1.8 - (2023-05-09)

[3bf60bc](https://github.com/tokio-rs/console/commit/3bf60bce7b478c189a3145311e06f14cb2fc1e11)...[5900300](https://github.com/tokio-rs/console/commit/59003004a6f2f2857be267061f23d34e2257e0f0)


### Added

- Reduce decimal digits in UI ([#402](https://github.com/tokio-rs/console/issues/402)) ([57b866d](https://github.com/tokio-rs/console/commit/57b866dd70ee36545ea0c02b02be872183cfa431))
- Use tokio task ids in task views ([#403](https://github.com/tokio-rs/console/issues/403)) ([001fc49](https://github.com/tokio-rs/console/commit/001fc49f09ad78cc4ab50770cf4a677ae177103f))
- Add support for Unix domain sockets ([#388](https://github.com/tokio-rs/console/issues/388)) ([bff8b8a](https://github.com/tokio-rs/console/commit/bff8b8a4291b0584ab4f97c5f91246eb9a68f262), closes [#296](https://github.com/tokio-rs/console/issues/296))
- Add scheduled time per task ([#406](https://github.com/tokio-rs/console/issues/406)) ([ac20daa](https://github.com/tokio-rs/console/commit/ac20daaf301f80e87002593813965d11d11371e4))
- Add task scheduled times histogram ([#409](https://github.com/tokio-rs/console/issues/409)) ([3b37dda](https://github.com/tokio-rs/console/commit/3b37dda773f8cd237f6759d193fdc83a75ab7653))
- Update `tonic` to 0.9 ([#420](https://github.com/tokio-rs/console/issues/420)) ([54f6be9](https://github.com/tokio-rs/console/commit/54f6be985a248d3dd5a98a7624a2447d0547bc60))
- Update MSRV to Rust 1.60.0 ([e3c5656](https://github.com/tokio-rs/console/commit/e3c56561a062be123be460dd477f512a6a9ec3cd))

### Documented

- Update screenshots in README ([#419](https://github.com/tokio-rs/console/issues/419)) ([4f71484](https://github.com/tokio-rs/console/commit/4f714845f3cda0978b0b1cc850072878ed4f8599))
- Revert "update screenshots in README ([#419](https://github.com/tokio-rs/console/issues/419))" ([7b86f7f](https://github.com/tokio-rs/console/commit/7b86f7f7d22d71ad7b677cdb2634c142c9bf5206))
- Update screenshots in README ([#421](https://github.com/tokio-rs/console/issues/421)) ([f4d3213](https://github.com/tokio-rs/console/commit/f4d321397355a6e458e170e174ac420c26e6353a))

### Fixed

- Fix ascii-only flipped input ([#377](https://github.com/tokio-rs/console/issues/377)) ([da0e972](https://github.com/tokio-rs/console/commit/da0e9724fa132595e2085cfb08ac7bfbf10542ba))
- Declare `tokio-console` bin as `default-run` ([#379](https://github.com/tokio-rs/console/issues/379)) ([40f7971](https://github.com/tokio-rs/console/commit/40f7971d30451f7321b73a03222b71731dabc52a))
- Make `retain_for` default to 6s if not specfied ([#383](https://github.com/tokio-rs/console/issues/383)) ([3248caa](https://github.com/tokio-rs/console/commit/3248caa8f8551e22c9d361e23cabd3c98aa143b6), fixes [#382](https://github.com/tokio-rs/console/issues/382))
- Enable view-switching keystrokes on details views ([#387](https://github.com/tokio-rs/console/issues/387)) ([d98f159](https://github.com/tokio-rs/console/commit/d98f15956075a2d64f5cb96b1011eff7b3110e51))
- Fix `ViewOptions` default lang' ([#394](https://github.com/tokio-rs/console/issues/394)) ([a7548d0](https://github.com/tokio-rs/console/commit/a7548d089812ac61602a31a699d14777d312ac6d), fixes [#393](https://github.com/tokio-rs/console/issues/393))
- Fix calculation of busy time during poll ([#405](https://github.com/tokio-rs/console/issues/405)) ([6fa2185](https://github.com/tokio-rs/console/commit/6fa2185134c8791446a1f1b5dc2ee11d254966ad))

## tokio-console-v0.1.7 - (2022-08-10)

[ce901c8](https://github.com/tokio-rs/console/commit/ce901c8f359d0de99430b51abd0cde63513de66a)...[3bf60bc](https://github.com/tokio-rs/console/commit/3bf60bce7b478c189a3145311e06f14cb2fc1e11)


### Added

- Emit a parse error a config file contains unknown fields ([#330](https://github.com/tokio-rs/console/issues/330)) ([3a67d47](https://github.com/tokio-rs/console/commit/3a67d476835e4a1d3557190b85f5a89c760490bb))
- Add missing configurations to config file ([#334](https://github.com/tokio-rs/console/issues/334)) ([472ff52](https://github.com/tokio-rs/console/commit/472ff52e6445dd2c103b218d60a4e0cad9a1972e), closes [#331](https://github.com/tokio-rs/console/issues/331))
- Display outliers in histogram view ([#351](https://github.com/tokio-rs/console/issues/351)) ([dec891f](https://github.com/tokio-rs/console/commit/dec891ff080b135dc10919b2b59665989e73daf3))
- Add subcommand to gen shell completions ([#336](https://github.com/tokio-rs/console/issues/336)) ([df4d468](https://github.com/tokio-rs/console/commit/df4d468375138b4115fb489b67ca72fbbd8f9ba1))
- Only suggest opening issues for panics ([#365](https://github.com/tokio-rs/console/issues/365)) ([23cb6bf](https://github.com/tokio-rs/console/commit/23cb6bf7cdaafd3fe691e4a6f7f91cc17e169795))
- Init error handling before subcmds ([#365](https://github.com/tokio-rs/console/issues/365)) ([6646568](https://github.com/tokio-rs/console/commit/66465689dceec509d9e1e37a55646a89285005e3))
- Filter out boring frames in backtraces ([#365](https://github.com/tokio-rs/console/issues/365)) ([523a44a](https://github.com/tokio-rs/console/commit/523a44a30cf047fe0a56f746624df0cc3239a160))
- Include config options in autogenerated issues ([#365](https://github.com/tokio-rs/console/issues/365)) ([fcb54df](https://github.com/tokio-rs/console/commit/fcb54dffda2a9f4c85cc82a24bff26e0777ceacc))

### Documented

- Update minimal Rust version ([#338](https://github.com/tokio-rs/console/issues/338)) ([ff3b6db](https://github.com/tokio-rs/console/commit/ff3b6db6fa06456a14992663e8ff7ba8c80c1cc1))

### Fixed

- Always log to a file instead of `stderr` ([#340](https://github.com/tokio-rs/console/issues/340)) ([ef39b9a](https://github.com/tokio-rs/console/commit/ef39b9a6419227d10e7a4d299ca95673ea200944), fixes [#339](https://github.com/tokio-rs/console/issues/339))
- Default `--no_colors` to `false` ([#344](https://github.com/tokio-rs/console/issues/344)) ([e58352f](https://github.com/tokio-rs/console/commit/e58352fe5e205620f1fe43acaceaff9cf7913394))

## tokio-console-v0.1.4 - (2022-04-13)

[3c55912](https://github.com/tokio-rs/console/commit/3c559121e3c5ad175471718a3cf87ada0146a7cd)...[ce901c8](https://github.com/tokio-rs/console/commit/ce901c8f359d0de99430b51abd0cde63513de66a)

### <a id = "tokio-console-v0.1.4-breaking"></a>Breaking Changes
- **Update `tonic` to `0.7` ([#318](https://github.com/tokio-rs/console/issues/318))** ([83d8a87](https://github.com/tokio-rs/console/commit/83d8a870bcc40be71bc23d0f45fc374899c636a8))<br />`console-api` is now no longer compatible with projects using `prost`
0.9 or `tonic` 0.7. These crates must be updated to use `console-api`
0.2.

### Added

- [**breaking**](#tokio-console-v0.1.4-breaking) Update `tonic` to `0.7` ([#318](https://github.com/tokio-rs/console/issues/318)) ([83d8a87](https://github.com/tokio-rs/console/commit/83d8a870bcc40be71bc23d0f45fc374899c636a8))
- Read configuration options from a config file ([#320](https://github.com/tokio-rs/console/issues/320)) ([defe346](https://github.com/tokio-rs/console/commit/defe34609508086cdc527fbf813cbca4732d49cd), closes [#310](https://github.com/tokio-rs/console/issues/310))
- Surface dropped event count if there are any ([#316](https://github.com/tokio-rs/console/issues/316)) ([16df5d3](https://github.com/tokio-rs/console/commit/16df5d30a011fc627e993ed71889877e70192baf))
- Add `gen-config` subcommand to generate a config file ([#324](https://github.com/tokio-rs/console/issues/324)) ([e034f8d](https://github.com/tokio-rs/console/commit/e034f8d0967fc589e4a48425075b6ae9abb47fc8))
- Add autogenerated example config file to docs ([#327](https://github.com/tokio-rs/console/issues/327)) ([79da280](https://github.com/tokio-rs/console/commit/79da280f4df9d1e0e6e3940e40b3faf123435a74))

### Documented

- Add tokio-console installation section ([#313](https://github.com/tokio-rs/console/issues/313)) ([d793903](https://github.com/tokio-rs/console/commit/d79390303590a534a8224efbe96c6023c336a32f))

## tokio-console-v0.1.3 - (2022-03-09)

[900a5c2](https://github.com/tokio-rs/console/commit/900a5c2bd5b610e9b939a5f824af1ac1a11267d0)...[3c55912](https://github.com/tokio-rs/console/commit/3c559121e3c5ad175471718a3cf87ada0146a7cd)


### Added

- Add icon representing column sorting state ([#301](https://github.com/tokio-rs/console/issues/301)) ([b9e0a22](https://github.com/tokio-rs/console/commit/b9e0a2266c98cd11a5261323dc20e04f17514b97))

### Fixed

- Prevent panics if subscriber reports out-of-order times ([#295](https://github.com/tokio-rs/console/issues/295)) ([80d7f42](https://github.com/tokio-rs/console/commit/80d7f4250ee5add0965ff100668be21d20621114))
- Exit crossterm before printing panic messages ([#307](https://github.com/tokio-rs/console/issues/307)) ([43606b9](https://github.com/tokio-rs/console/commit/43606b9a7ccff0157325effbc48e1d71a194e5de))

## tokio-console-v0.1.2 - (2022-02-18)

[e7b228d](https://github.com/tokio-rs/console/commit/e7b228d13b5da3885532ff5d42d7f41c90dcbcb0)...[900a5c2](https://github.com/tokio-rs/console/commit/900a5c2bd5b610e9b939a5f824af1ac1a11267d0)


### Added

- Fix missing histogram in task details ([#269](https://github.com/tokio-rs/console/issues/269)) ([884f4ec](https://github.com/tokio-rs/console/commit/884f4ecac8cba7eee7f895024da4c6e28de75289))

### Documented

- Document minimum Tokio versions ([#291](https://github.com/tokio-rs/console/issues/291)) ([3b1f14a](https://github.com/tokio-rs/console/commit/3b1f14a50c507e7b5b672491fada6dfb067fc671), closes [#281](https://github.com/tokio-rs/console/issues/281))

### Fixed

- Console-api dependencies to require 0.1.2 ([#274](https://github.com/tokio-rs/console/issues/274)) ([b95f683](https://github.com/tokio-rs/console/commit/b95f683f0514978429535a75c86f8974b05a69aa))

<!-- generated by git-cliff -->
