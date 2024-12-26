# Changelog

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.9.0 - (2024-12-26)


### Added

- Add the WatchState API (#582) ([7c1f9f2](https://github.com/tokio-rs/console/commit/7c1f9f216f499a0309ecf597c721252186e72c82))

### Fixed

- Bump minimum version of tonic (#593) ([1f41b61](https://github.com/tokio-rs/console/commit/1f41b61fbada02d84cfa85911e66eb0078cae596))


## 0.8.1 - (2024-10-24)

_No outward facing changes_

## 0.8.0 - (2024-07-29)

### <a id = "0.8.0-breaking"></a>Breaking Changes
- **Upgrade tonic to 0.12 ([#571](https://github.com/tokio-rs/console/pull/571))** ([5f6faa2](https://github.com/tokio-rs/console/commit/5f6faa22d944735c2b8c312cac03b35a4ab228ef))<br />This is a breaking change for users of `console-api` and
`console-subscriber`, as it changes the public `tonic`, `prost` and
`prost-types` dependency to a semver-incompatible version. This breaks
compatibility with `tonic` 0.11.x as well as `prost`/`prost-types`
0.12.x.

### Updated

- [**breaking**](#0.8.0-breaking) Upgrade tonic to 0.12 ([#571](https://github.com/tokio-rs/console/pull/571)) ([5f6faa2](https://github.com/tokio-rs/console/commit/5f6faa22d944735c2b8c312cac03b35a4ab228ef))


## console-api-v0.7.0 - (2024-06-10)

### <a id = "0.7.0-breaking"></a>Breaking Changes
- **Bump tonic to 0.11 ([#547](https://github.com/tokio-rs/console/pull/547))** ([ef6816c](https://github.com/tokio-rs/console/commit/ef6816caa0fe84171105b513425506f25d3082af))<br />This is a breaking change for users of `console-api` and
`console-subscriber`, as it changes the public `tonic` dependency to a
semver-incompatible version. This breaks compatibility with `tonic`
0.10.x.

### Documented

- Fix typo in proto ([#472](https://github.com/tokio-rs/console/pull/472)) ([2dd3559](https://github.com/tokio-rs/console/commit/2dd3559ccf8a88e0e0a140f076135ea3f6f26f02))

### Updated

- [**breaking**](#0.7.0-breaking) Bump tonic to 0.11 ([#547](https://github.com/tokio-rs/console/pull/547)) ([ef6816c](https://github.com/tokio-rs/console/commit/ef6816caa0fe84171105b513425506f25d3082af))


## console-api-v0.6.0 - (2023-09-29)

### <a id = "console-api-v0.6.0-breaking"></a>Breaking Changes
- **Update `tonic` to v0.10 and increase MSRV to 1.64 ([#464](https://github.com/tokio-rs/console/issues/464))** ([882318a](https://github.com/tokio-rs/console/commit/882318a006d060c763f97afa7e03a45ef9736fe6))<br />This is a breaking change for users of `console-api` and
`console-subscriber`, as it changes the public `tonic` dependency to a
semver-incompatible version. This breaks compatibility with `tonic`
0.9.x and `prost` 0.11.x.

### Added

- [**breaking**](#console-api-v0.6.0-breaking) Update `tonic` to v0.10 and increase MSRV to 1.64 ([#464](https://github.com/tokio-rs/console/issues/464)) ([882318a](https://github.com/tokio-rs/console/commit/882318a006d060c763f97afa7e03a45ef9736fe6))

### Documented

- Update MSRV version docs to 1.64 ([#467](https://github.com/tokio-rs/console/issues/467)) ([a7acbcc](https://github.com/tokio-rs/console/commit/a7acbcc966ef61825f67d93988add8643be760d5))

### Fixed

- Add explicit `futures-core` dep to fix broken builds ([#453](https://github.com/tokio-rs/console/issues/453)) ([88638f9](https://github.com/tokio-rs/console/commit/88638f992c3ada6c97ca1921c66a3a4bbf5b23c1))

## console-api-v0.5.0 - (2023-09-29)

### <a id = "console-api-v0.5.0-breaking"></a>Breaking Changes
- **Update `tonic` to v0.9 ([#420](https://github.com/tokio-rs/console/issues/420))** ([b70c1d8](https://github.com/tokio-rs/console/commit/b70c1d886d64fc43de6715f07ae49313f778e92b))<br />This is a breaking change for users of `console-api`, as it changes the
public `tonic` dependency to a semver-incompatible version. This breaks
compatibility with `tonic` 0.8.

### Added

- Use tokio task ids in task views ([#403](https://github.com/tokio-rs/console/issues/403)) ([001fc49](https://github.com/tokio-rs/console/commit/001fc49f09ad78cc4ab50770cf4a677ae177103f))
- Add scheduled time per task ([#406](https://github.com/tokio-rs/console/issues/406)) ([ac20daa](https://github.com/tokio-rs/console/commit/ac20daaf301f80e87002593813965d11d11371e4))
- Add task scheduled times histogram ([#409](https://github.com/tokio-rs/console/issues/409)) ([3b37dda](https://github.com/tokio-rs/console/commit/3b37dda773f8cd237f6759d193fdc83a75ab7653))
- [**breaking**](#console-api-v0.5.0-breaking) Update `tonic` to v0.9 ([#420](https://github.com/tokio-rs/console/issues/420)) ([b70c1d8](https://github.com/tokio-rs/console/commit/b70c1d886d64fc43de6715f07ae49313f778e92b))
- Update MSRV to Rust 1.60.0 ([e3c5656](https://github.com/tokio-rs/console/commit/e3c56561a062be123be460dd477f512a6a9ec3cd))

## console-api-v0.4.0 - (2023-09-29)

### <a id = "console-api-v0.4.0-breaking"></a>Breaking Changes
- **Update Tonic and Prost dependencies ([#364](https://github.com/tokio-rs/console/issues/364))** ([f9b8e03](https://github.com/tokio-rs/console/commit/f9b8e03bd7ee1d0edb441c94a93a350d5b06ed3b))<br />This commit updates the public dependencies `prost` and `tonic` to
semver-incompatible versions (v0.11.0 and v0.8.0, respectively). This is
a breaking change for users who are integrating the `console-api` protos
with their own `tonic` servers or clients.

### Added

- [**breaking**](#console-api-v0.4.0-breaking) Update Tonic and Prost dependencies ([#364](https://github.com/tokio-rs/console/issues/364)) ([f9b8e03](https://github.com/tokio-rs/console/commit/f9b8e03bd7ee1d0edb441c94a93a350d5b06ed3b))

## console-api-v0.6.0 - (2023-09-29)

[2cb6ee5](https://github.com/tokio-rs/console/commit/2cb6ee5b813837324f5f9934a929ac928cfbb03f)...[a7acbcc](https://github.com/tokio-rs/console/commit/a7acbcc966ef61825f67d93988add8643be760d5)

### <a id = "console-api-v0.6.0-breaking"></a>Breaking Changes
- **Update `tonic` to v0.9 ([#420](https://github.com/tokio-rs/console/issues/420))** ([b70c1d8](https://github.com/tokio-rs/console/commit/b70c1d886d64fc43de6715f07ae49313f778e92b))<br />This is a breaking change for users of `console-api`, as it changes the
public `tonic` dependency to a semver-incompatible version. This breaks
compatibility with `tonic` 0.8.
- **Update `tonic` to v0.10 and increase MSRV to 1.64 ([#464](https://github.com/tokio-rs/console/issues/464))** ([882318a](https://github.com/tokio-rs/console/commit/882318a006d060c763f97afa7e03a45ef9736fe6))<br />This is a breaking change for users of `console-api` and
`console-subscriber`, as it changes the public `tonic` dependency to a
semver-incompatible version. This breaks compatibility with `tonic`
0.9.x and `prost` 0.11.x.

### Added

- Use tokio task ids in task views ([#403](https://github.com/tokio-rs/console/issues/403)) ([001fc49](https://github.com/tokio-rs/console/commit/001fc49f09ad78cc4ab50770cf4a677ae177103f))
- Add scheduled time per task ([#406](https://github.com/tokio-rs/console/issues/406)) ([ac20daa](https://github.com/tokio-rs/console/commit/ac20daaf301f80e87002593813965d11d11371e4))
- Add task scheduled times histogram ([#409](https://github.com/tokio-rs/console/issues/409)) ([3b37dda](https://github.com/tokio-rs/console/commit/3b37dda773f8cd237f6759d193fdc83a75ab7653))
- [**breaking**](#console-api-v0.6.0-breaking) Update `tonic` to v0.9 ([#420](https://github.com/tokio-rs/console/issues/420)) ([b70c1d8](https://github.com/tokio-rs/console/commit/b70c1d886d64fc43de6715f07ae49313f778e92b))
- Update MSRV to Rust 1.60.0 ([e3c5656](https://github.com/tokio-rs/console/commit/e3c56561a062be123be460dd477f512a6a9ec3cd))
- [**breaking**](#console-api-v0.6.0-breaking) Update `tonic` to v0.10 and increase MSRV to 1.64 ([#464](https://github.com/tokio-rs/console/issues/464)) ([882318a](https://github.com/tokio-rs/console/commit/882318a006d060c763f97afa7e03a45ef9736fe6))

### Documented

- Update MSRV version docs to 1.64 ([#467](https://github.com/tokio-rs/console/issues/467)) ([a7acbcc](https://github.com/tokio-rs/console/commit/a7acbcc966ef61825f67d93988add8643be760d5))

### Fixed

- Add explicit `futures-core` dep to fix broken builds ([#453](https://github.com/tokio-rs/console/issues/453)) ([88638f9](https://github.com/tokio-rs/console/commit/88638f992c3ada6c97ca1921c66a3a4bbf5b23c1))

## console-api-v0.3.0 - (2022-05-23)

[0d6d7a9](https://github.com/tokio-rs/console/commit/0d6d7a9af3a8174ca624f4289c5877ad3ac4f227)...[5490d64](https://github.com/tokio-rs/console/commit/5490d64c098d6997f4327e7ec08d5136ece2a2e5)

### <a id = "console-api-v0.3.0-breaking"></a>Breaking Changes
- **Add optional histogram outlier details ([#351](https://github.com/tokio-rs/console/issues/351))** ([4611591](https://github.com/tokio-rs/console/commit/46115913877051090abd36719161f306b68124c7))<br />This is a breaking change *to the Rust bindings* (the `console-api`
crate) due to changing a field from an `Option` to a protobuf `oneof`
(introducing a new enum type). This is **not** a breaking change to the
protobufs themselves --- the actual wire format change is
backwards-compatible, but the generated Rust code changes in a breaking
way.

### Added

- [**breaking**](#console-api-v0.3.0-breaking) Add optional histogram outlier details ([#351](https://github.com/tokio-rs/console/issues/351)) ([4611591](https://github.com/tokio-rs/console/commit/46115913877051090abd36719161f306b68124c7))

### Documented

- Update minimal Rust version ([#338](https://github.com/tokio-rs/console/issues/338)) ([ff3b6db](https://github.com/tokio-rs/console/commit/ff3b6db6fa06456a14992663e8ff7ba8c80c1cc1))

## console-api-v0.2.0 - (2022-04-11)

[c7cab71](https://github.com/tokio-rs/console/commit/c7cab7112368682a8ccea8c4ec4a5ef99b88d567)...[0d6d7a9](https://github.com/tokio-rs/console/commit/0d6d7a9af3a8174ca624f4289c5877ad3ac4f227)

### <a id = "console-api-v0.2.0-breaking"></a>Breaking Changes
- **Update `tonic` to `0.7` ([#318](https://github.com/tokio-rs/console/issues/318))** ([83d8a87](https://github.com/tokio-rs/console/commit/83d8a870bcc40be71bc23d0f45fc374899c636a8))<br />`console-api` is now no longer compatible with projects using `prost`
0.9 or `tonic` 0.7. These crates must be updated to use `console-api`
0.2.

### Added

- [**breaking**](#console-api-v0.2.0-breaking) Update `tonic` to `0.7` ([#318](https://github.com/tokio-rs/console/issues/318)) ([83d8a87](https://github.com/tokio-rs/console/commit/83d8a870bcc40be71bc23d0f45fc374899c636a8))

### Documented

- Reword comment on the tracing_core::metadata::Kind match ([#272](https://github.com/tokio-rs/console/issues/272)) ([1ac3b9f](https://github.com/tokio-rs/console/commit/1ac3b9f4558d8f4f1233aa40ffd87702c58cbfee))

## console-api-v0.1.2 - (2022-02-04)

[1fe0650](https://github.com/tokio-rs/console/commit/1fe06508604dcfff473455fe848e9ff2a5588f62)...[c7cab71](https://github.com/tokio-rs/console/commit/c7cab7112368682a8ccea8c4ec4a5ef99b88d567)


### Fixed

- Fix accidental exhaustive matching on `metadata::Kind` ([#271](https://github.com/tokio-rs/console/issues/271)) ([d9aafaa](https://github.com/tokio-rs/console/commit/d9aafaa05549379cf02113faea90816de2235c16), fixes [#270](https://github.com/tokio-rs/console/issues/270))

## console-api-v0.1.1 - (2022-01-18)

[5c041d7](https://github.com/tokio-rs/console/commit/5c041d7149684fbc2735058c386f85e02b5381fb)...[1fe0650](https://github.com/tokio-rs/console/commit/1fe06508604dcfff473455fe848e9ff2a5588f62)


### Documented

- Post-release readme fixup ([#221](https://github.com/tokio-rs/console/issues/221)) ([28a4321](https://github.com/tokio-rs/console/commit/28a4321e0f555c3744194ec64dccc93e4fd194ce))

## console-api-v0.1.0 - (2021-12-16)


### Added

- Add TUI app, simple top-style view ([#2](https://github.com/tokio-rs/console/issues/2)) ([c7f0b43](https://github.com/tokio-rs/console/commit/c7f0b43494e439331ea2ae0ba4fc4cea8ddff6e3))
- Send structured fields on the wire ([#26](https://github.com/tokio-rs/console/issues/26)) ([38adbd9](https://github.com/tokio-rs/console/commit/38adbd97aefc53d06e509c7b33c98b4dcfa7a970), fixes [#6](https://github.com/tokio-rs/console/issues/6))
- Populate `Metadata`'s `field names` ([#32](https://github.com/tokio-rs/console/issues/32)) ([e45fca0](https://github.com/tokio-rs/console/commit/e45fca08102cefec7494d28f80863990cfb24160))
- Record and send poll times with HdrHistogram ([#47](https://github.com/tokio-rs/console/issues/47)) ([94e7834](https://github.com/tokio-rs/console/commit/94e7834db44c3b19c54ff16a22f1b0e6464be1a2), closes [#36](https://github.com/tokio-rs/console/issues/36))
- Use sequential `u64` task IDs ([#75](https://github.com/tokio-rs/console/issues/75)) ([c2c486e](https://github.com/tokio-rs/console/commit/c2c486ee9c792453db81786490bff52a031be9e9))
- Resource instrumentation ([#77](https://github.com/tokio-rs/console/issues/77)) ([f4a21ac](https://github.com/tokio-rs/console/commit/f4a21acb18935af8b256999e2380eb5fb7e17d72))
- Use `Location` for tasks and resources ([#154](https://github.com/tokio-rs/console/issues/154)) ([08c5186](https://github.com/tokio-rs/console/commit/08c5186eb01f18f8e4018058d12817e4127dd7be))
- Add resource detail view ([#188](https://github.com/tokio-rs/console/issues/188)) ([1aa9b59](https://github.com/tokio-rs/console/commit/1aa9b594f30e42098c6c6bbf41eb1d2b01dc0426))
- Count dropped events due to buffer cap ([#211](https://github.com/tokio-rs/console/issues/211)) ([aa09600](https://github.com/tokio-rs/console/commit/aa09600b3bdc6591eafc9fe7b4507f7da2bca498))

### Documented

- Console-api docs ([#197](https://github.com/tokio-rs/console/issues/197)) ([fdf8637](https://github.com/tokio-rs/console/commit/fdf8637f2671a95d84a4c9046a2ed411e08045ef))
- Add a README and lib.rs docs ([#201](https://github.com/tokio-rs/console/issues/201)) ([5af6e07](https://github.com/tokio-rs/console/commit/5af6e07d6eb44b133dcd0d6deff6b99a806d9e79))
- Add a README (and `lib.rs` docs) ([#202](https://github.com/tokio-rs/console/issues/202)) ([a79c505](https://github.com/tokio-rs/console/commit/a79c5056875a3593b4fd61d18e42c2aa6a08688c))

### Fixed

- Make proto/ vendor-able ([#128](https://github.com/tokio-rs/console/issues/128)) ([81cd611](https://github.com/tokio-rs/console/commit/81cd61152755abfdfa2f00727d079e65006e8c55))

<!-- generated by git-cliff -->
