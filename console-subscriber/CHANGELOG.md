# Changelog

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.2 - (2024-12-26)


### Added

- Add the WatchState API (#582) ([7c1f9f2](https://github.com/tokio-rs/console/commit/7c1f9f216f499a0309ecf597c721252186e72c82))

### Documented

- Fix typo in doc comment ([3dbca7a](https://github.com/tokio-rs/console/commit/3dbca7a76e581d15ac598adc82e0c465d530de36))


## 0.4.1 - (2024-10-24)


### Added

- Add large future lints ([#587](https://github.com/tokio-rs/console/pull/587)) ([ae17230](https://github.com/tokio-rs/console/commit/ae1723091fcc76597e78bae39129a48d8cd515c9))


## 0.4.0 - (2024-07-29)

### <a id = "0.4.0-breaking"></a>Breaking Changes
- **Upgrade tonic to 0.12 ([#571](https://github.com/tokio-rs/console/pull/571))** ([5f6faa2](https://github.com/tokio-rs/console/commit/5f6faa22d944735c2b8c312cac03b35a4ab228ef))<br />This is a breaking change for users of `console-api` and
`console-subscriber`, as it changes the public `tonic`, `prost` and
`prost-types` dependency to a semver-incompatible version. This breaks
compatibility with `tonic` 0.11.x as well as `prost`/`prost-types`
0.12.x.

### Added

- Add `TOKIO_CONSOLE_BUFFER_CAPACITY`  env variable ([#568](https://github.com/tokio-rs/console/pull/568)) ([a6cf14b](https://github.com/tokio-rs/console/commit/a6cf14b370275367dcecf1191e60f0bd260250d8))

### Fixed

- Remove unused `AggregatorHandle` and fix other lints ([#578](https://github.com/tokio-rs/console/pull/578)) ([c442063](https://github.com/tokio-rs/console/commit/c44206307997f8fc9ae173c466faf89c8f25c4b0))

### Updated

- [**breaking**](#0.4.0-breaking) Upgrade tonic to 0.12 ([#571](https://github.com/tokio-rs/console/pull/571)) ([5f6faa2](https://github.com/tokio-rs/console/commit/5f6faa22d944735c2b8c312cac03b35a4ab228ef))


## console-subscriber-v0.3.0 - (2024-06-10)

### <a id = "0.3.0-breaking"></a>Breaking Changes
- **Bump tonic to 0.11 ([#547](https://github.com/tokio-rs/console/pull/547))** ([ef6816c](https://github.com/tokio-rs/console/commit/ef6816caa0fe84171105b513425506f25d3082af))<br />This is a breaking change for users of `console-api` and
`console-subscriber`, as it changes the public `tonic` dependency to a
semver-incompatible version. This breaks compatibility with `tonic`
0.10.x.

### Added

- Replace target column with kind column in tasks view ([#478](https://github.com/tokio-rs/console/pull/478)) ([903d9fa](https://github.com/tokio-rs/console/commit/903d9fa9f9d2dddec2235206b792c264ed9892fb))
- Reduce retention period to fit in max message size ([#503](https://github.com/tokio-rs/console/pull/503)) ([bd3dd71](https://github.com/tokio-rs/console/commit/bd3dd71eb0645c028858967ed5b3f14ed34d0605))
- Support grpc-web and add `grpc-web` feature ([#498](https://github.com/tokio-rs/console/pull/498)) ([4150253](https://github.com/tokio-rs/console/commit/41502531396106b551a9dde2d3a83ddb0beac774))

### Documented

- Add a grpc-web app example ([#526](https://github.com/tokio-rs/console/pull/526)) ([4af30f2](https://github.com/tokio-rs/console/commit/4af30f2c1df919a1e0d4f448534d15b4a1bb836b))
- Fix typo in doc comment ([#543](https://github.com/tokio-rs/console/pull/543)) ([ff22678](https://github.com/tokio-rs/console/commit/ff2267880ba96f4fdf32090e05b66cf80189d7f8))

### Fixed

- Don't save poll_ops if no-one is receiving them ([#501](https://github.com/tokio-rs/console/pull/501)) ([1656c79](https://github.com/tokio-rs/console/commit/1656c791af35bb0500bb6dd3c60344a0ceb12520))
- Ignore metadata that is not a span or event ([#554](https://github.com/tokio-rs/console/pull/554)) ([852a977](https://github.com/tokio-rs/console/commit/852a977bab71d0f6ae47c6c5c1c20b8679c9e576))

### Updated

- [**breaking**](#0.3.0-breaking) Bump tonic to 0.11 ([#547](https://github.com/tokio-rs/console/pull/547)) ([ef6816c](https://github.com/tokio-rs/console/commit/ef6816caa0fe84171105b513425506f25d3082af))


## console-subscriber-v0.2.0 - (2023-09-29)

[c8c4a85](https://github.com/tokio-rs/console/commit/c8c4a85df2da55c9745df6f38e19631e84ed0cf5)...[c8c4a85](https://github.com/tokio-rs/console/commit/c8c4a85df2da55c9745df6f38e19631e84ed0cf5)

### <a id = "console-subscriber-v0.2.0-breaking"></a>Breaking Changes
- **Update Tonic and Prost dependencies ([#364](https://github.com/tokio-rs/console/issues/364))** ([f9b8e03](https://github.com/tokio-rs/console/commit/f9b8e03bd7ee1d0edb441c94a93a350d5b06ed3b))<br />This commit updates the public dependencies `prost` and `tonic` to
semver-incompatible versions (v0.11.0 and v0.8.0, respectively). This is
a breaking change for users who are integrating the `console-api` protos
with their own `tonic` servers or clients.
- **Update `tonic` to v0.10 and increase MSRV to 1.64 ([#464](https://github.com/tokio-rs/console/issues/464))** ([96e62c8](https://github.com/tokio-rs/console/commit/96e62c83ef959569bb062dc8fee98fa2b2461e8d))<br />This is a breaking change for users of `console-api` and
`console-subscriber`, as it changes the public `tonic` dependency to a
semver-incompatible version. This breaks compatibility with `tonic`
0.9.x and `prost` 0.11.x.

### Added

- [**breaking**](#console-subscriber-v0.2.0-breaking) Update Tonic and Prost dependencies ([#364](https://github.com/tokio-rs/console/issues/364)) ([f9b8e03](https://github.com/tokio-rs/console/commit/f9b8e03bd7ee1d0edb441c94a93a350d5b06ed3b))
- Add support for Unix domain sockets ([#388](https://github.com/tokio-rs/console/issues/388)) ([a944dbc](https://github.com/tokio-rs/console/commit/a944dbcff2de49e45d5fa99edb227c85a5c3d40f), closes [#296](https://github.com/tokio-rs/console/issues/296))
- Add scheduled time per task ([#406](https://github.com/tokio-rs/console/issues/406)) ([f280df9](https://github.com/tokio-rs/console/commit/f280df94100d24e868ce3f9fbfec160677d8a124))
- Add task scheduled times histogram ([#409](https://github.com/tokio-rs/console/issues/409)) ([d92a399](https://github.com/tokio-rs/console/commit/d92a39994f6e759ddba4e53ab7263a0c4edb0b67))
- Update `tonic` to 0.9 ([#420](https://github.com/tokio-rs/console/issues/420)) ([48af1ee](https://github.com/tokio-rs/console/commit/48af1eef6352bd35c607267d68b24cf16033beeb))
- Update MSRV to Rust 1.60.0 ([b18ee47](https://github.com/tokio-rs/console/commit/b18ee473aa499aa581117baea7404623d98b081c))
- Expose server parts ([#451](https://github.com/tokio-rs/console/issues/451)) ([e51ac5a](https://github.com/tokio-rs/console/commit/e51ac5a15338631136cc2d0e285ec3a9337c8ce4))
- Add cfg `console_without_tokio_unstable` ([#446](https://github.com/tokio-rs/console/issues/446)) ([7ed6673](https://github.com/tokio-rs/console/commit/7ed6673241b0a566c00be59f7a0cbc911ea6a165))
- Add warning for tasks that never yield ([#439](https://github.com/tokio-rs/console/issues/439)) ([d05fa9e](https://github.com/tokio-rs/console/commit/d05fa9ee6456dd9a9eec72c5299c32a4f0c845c0))
- [**breaking**](#console-subscriber-v0.2.0-breaking) Update `tonic` to v0.10 and increase MSRV to 1.64 ([#464](https://github.com/tokio-rs/console/issues/464)) ([96e62c8](https://github.com/tokio-rs/console/commit/96e62c83ef959569bb062dc8fee98fa2b2461e8d))

### Documented

- Fix unclosed code block ([#463](https://github.com/tokio-rs/console/issues/463)) ([362bdbe](https://github.com/tokio-rs/console/commit/362bdbea1af1e36ec139ad460e97f0eeea79a9f2))
- Update MSRV version docs to 1.64 ([#467](https://github.com/tokio-rs/console/issues/467)) ([94a5a51](https://github.com/tokio-rs/console/commit/94a5a5117b85e723c28fafa1eadabf31057570c3))

### Fixed

- Fix build on tokio 1.21.0 ([#374](https://github.com/tokio-rs/console/issues/374)) ([c34ac2d](https://github.com/tokio-rs/console/commit/c34ac2d6a569b0fd02b9b78ff4ecffd019d30a87))
- Fix off-by-one indexing for `callsites` ([#391](https://github.com/tokio-rs/console/issues/391)) ([43891ab](https://github.com/tokio-rs/console/commit/43891aba0a42ec85cdcdfeac2a31ffe612eb1841))
- Bump minimum Tokio version ([#397](https://github.com/tokio-rs/console/issues/397)) ([bbb8f25](https://github.com/tokio-rs/console/commit/bbb8f25a666a4e15de3c5054244e228a51b5c7c0), fixes [#386](https://github.com/tokio-rs/console/issues/386))
- Fix self wakes count ([#430](https://github.com/tokio-rs/console/issues/430)) ([d308935](https://github.com/tokio-rs/console/commit/d3089350da7b483ef80284d42b4114bfc50c2b33))
- Remove clock skew warning in `start_poll` ([#434](https://github.com/tokio-rs/console/issues/434)) ([4a88b28](https://github.com/tokio-rs/console/commit/4a88b28e608465eb3c23cbe7f0cb589ed6110962))
- Do not report excessive polling ([#378](https://github.com/tokio-rs/console/issues/378)) ([#440](https://github.com/tokio-rs/console/issues/440)) ([8b483bf](https://github.com/tokio-rs/console/commit/8b483bf806bc5f5f7c94e97ea79299ae8ccb7955), closes [#378](https://github.com/tokio-rs/console/issues/378))
- Correct retain logic ([#447](https://github.com/tokio-rs/console/issues/447)) ([36ffc51](https://github.com/tokio-rs/console/commit/36ffc513b26f27d5fb6344c24f12572ec76e41ac))

## console-subscriber-v0.1.10 - (2023-07-03)

[05cdab0](https://github.com/tokio-rs/console/commit/05cdab07a3da603697520a56f0b99b2e2042d8bd)...[91929d0](https://github.com/tokio-rs/console/commit/91929d030768287b5f95595a757eea5eeb151022)


### Fixed

- Fix self wakes count ([#430](https://github.com/tokio-rs/console/issues/430)) ([ee0b8e2](https://github.com/tokio-rs/console/commit/ee0b8e28c7761edd277beb865b2a1e0a3bfa1851))
- Do not report excessive polling ([#378](https://github.com/tokio-rs/console/issues/378)) ([#440](https://github.com/tokio-rs/console/issues/440)) ([91929d0](https://github.com/tokio-rs/console/commit/91929d030768287b5f95595a757eea5eeb151022), closes [#378](https://github.com/tokio-rs/console/issues/378))

### Console_subscriber

- Remove clock skew warning in start_poll ([#434](https://github.com/tokio-rs/console/issues/434)) ([fb45ca1](https://github.com/tokio-rs/console/commit/fb45ca16a77a9a63e88494a892076d41495e6bb2))

## console-subscriber-v0.1.9 - (2023-05-09)

[8fb1732](https://github.com/tokio-rs/console/commit/8fb1732dfd78ec3a8e4945c453d1c127f63ecdc4)...[05cdab0](https://github.com/tokio-rs/console/commit/05cdab07a3da603697520a56f0b99b2e2042d8bd)


### Added

- Add support for Unix domain sockets ([#388](https://github.com/tokio-rs/console/issues/388)) ([bff8b8a](https://github.com/tokio-rs/console/commit/bff8b8a4291b0584ab4f97c5f91246eb9a68f262), closes [#296](https://github.com/tokio-rs/console/issues/296))
- Add scheduled time per task ([#406](https://github.com/tokio-rs/console/issues/406)) ([ac20daa](https://github.com/tokio-rs/console/commit/ac20daaf301f80e87002593813965d11d11371e4))
- Add task scheduled times histogram ([#409](https://github.com/tokio-rs/console/issues/409)) ([3b37dda](https://github.com/tokio-rs/console/commit/3b37dda773f8cd237f6759d193fdc83a75ab7653))
- Update `tonic` to 0.9 ([#420](https://github.com/tokio-rs/console/issues/420)) ([54f6be9](https://github.com/tokio-rs/console/commit/54f6be985a248d3dd5a98a7624a2447d0547bc60))
- Update MSRV to Rust 1.60.0 ([e3c5656](https://github.com/tokio-rs/console/commit/e3c56561a062be123be460dd477f512a6a9ec3cd))

### Fixed

- Fix off-by-one indexing for `callsites` ([#391](https://github.com/tokio-rs/console/issues/391)) ([3c668a3](https://github.com/tokio-rs/console/commit/3c668a3679b5536f8a047db7a35d432c645aacef))
- Bump minimum Tokio version ([#397](https://github.com/tokio-rs/console/issues/397)) ([7286d6f](https://github.com/tokio-rs/console/commit/7286d6f75022f3504a0379ff3fa15585a614753e), fixes [#386](https://github.com/tokio-rs/console/issues/386))

## console-subscriber-v0.1.8 - (2022-09-04)

[95a17b6](https://github.com/tokio-rs/console/commit/95a17b6f549ca6d9d22777043dc6f65432fdc69b)...[8fb1732](https://github.com/tokio-rs/console/commit/8fb1732dfd78ec3a8e4945c453d1c127f63ecdc4)


### Fixed

- Fix build on tokio 1.21.0 ([#374](https://github.com/tokio-rs/console/issues/374)) ([0106407](https://github.com/tokio-rs/console/commit/0106407cc712b65793801d70324896138d4a4d59))

## console-subscriber-v0.1.6 - (2022-05-23)

[0b3f592](https://github.com/tokio-rs/console/commit/0b3f59280070b1b9f44ec7473ff36279c4ad54c4)...[95a17b6](https://github.com/tokio-rs/console/commit/95a17b6f549ca6d9d22777043dc6f65432fdc69b)


### Added

- Add `Builder::poll_duration_histogram_max` ([#351](https://github.com/tokio-rs/console/issues/351)) ([a966feb](https://github.com/tokio-rs/console/commit/a966feb3d24e555b76c39830216f6fcff6c18f85))

### Fixed

- Fix memory leak from resizing histograms ([#351](https://github.com/tokio-rs/console/issues/351)) ([32dd337](https://github.com/tokio-rs/console/commit/32dd33760a633a409d7828783dd8c095c7b6b0ed), fixes [#350](https://github.com/tokio-rs/console/issues/350))

## console-subscriber-v0.1.5 - (2022-04-30)

[43fb91f](https://github.com/tokio-rs/console/commit/43fb91f58b1ed6255d21fe591c68275995ea8894)...[0b3f592](https://github.com/tokio-rs/console/commit/0b3f59280070b1b9f44ec7473ff36279c4ad54c4)


### Added

- Add support for `EnvFilter` in `Builder::init` ([#337](https://github.com/tokio-rs/console/issues/337)) ([1fe84b7](https://github.com/tokio-rs/console/commit/1fe84b7270e9e6d41d0f1b97029ef4793aa6b58d))

### Documented

- Fix links to console-subscriber's API docs ([#326](https://github.com/tokio-rs/console/issues/326)) ([bebaa16](https://github.com/tokio-rs/console/commit/bebaa16b3b72ea08724bc0dc5d3aae60920485c7))
- Fix broken `Server` rustdoc ([#332](https://github.com/tokio-rs/console/issues/332)) ([84605c4](https://github.com/tokio-rs/console/commit/84605c4adc809bd715670c61a8a6e1a33a790fdf))
- Update minimal Rust version ([#338](https://github.com/tokio-rs/console/issues/338)) ([ff3b6db](https://github.com/tokio-rs/console/commit/ff3b6db6fa06456a14992663e8ff7ba8c80c1cc1))

## console-subscriber-v0.1.4 - (2022-04-11)

[0e67d17](https://github.com/tokio-rs/console/commit/0e67d17e1b92f549c787a5c700008064c10da00e)...[43fb91f](https://github.com/tokio-rs/console/commit/43fb91f58b1ed6255d21fe591c68275995ea8894)

### <a id = "console-subscriber-v0.1.4-breaking"></a>Breaking Changes
- **Update `tonic` to `0.7` ([#318](https://github.com/tokio-rs/console/issues/318))** ([83d8a87](https://github.com/tokio-rs/console/commit/83d8a870bcc40be71bc23d0f45fc374899c636a8))<br />`console-api` is now no longer compatible with projects using `prost`
0.9 or `tonic` 0.7. These crates must be updated to use `console-api`
0.2.

### Added

- [**breaking**](#console-subscriber-v0.1.4-breaking) Update `tonic` to `0.7` ([#318](https://github.com/tokio-rs/console/issues/318)) ([83d8a87](https://github.com/tokio-rs/console/commit/83d8a870bcc40be71bc23d0f45fc374899c636a8))
- Don't trace tasks spawned through the console server ([#314](https://github.com/tokio-rs/console/issues/314)) ([0045e9b](https://github.com/tokio-rs/console/commit/0045e9bf509b8fd180c20ea846ff1da065c86a7f))

### Documented

- Warn against enabling compile time filters in the readme ([#317](https://github.com/tokio-rs/console/issues/317)) ([9a27cd2](https://github.com/tokio-rs/console/commit/9a27cd23dfe1004c5cc8e04c58dfac187ebf93fa), closes [#315](https://github.com/tokio-rs/console/issues/315))

### Fixed

- Fix memory leak from historical `PollOp`s ([#311](https://github.com/tokio-rs/console/issues/311)) ([9178ecf](https://github.com/tokio-rs/console/commit/9178ecf02f094f8b23dc26f02faaba4ec09fd8f5), fixes [#256](https://github.com/tokio-rs/console/issues/256))

## console-subscriber-v0.1.3 - (2022-02-18)

[e590df3](https://github.com/tokio-rs/console/commit/e590df39ca38cf795b1aec493403e1411e3b4766)...[0e67d17](https://github.com/tokio-rs/console/commit/0e67d17e1b92f549c787a5c700008064c10da00e)


### Added

- Add `Builder::filter_env_var` builder parameter ([#276](https://github.com/tokio-rs/console/issues/276)) ([dbdb149](https://github.com/tokio-rs/console/commit/dbdb14949bd2ac7c58e5c38cecbeb3fb76f45619), closes [#206](https://github.com/tokio-rs/console/issues/206))

### Documented

- Fix broken links in READMEs and subscriber doc comment ([#285](https://github.com/tokio-rs/console/issues/285)) ([a2202f7](https://github.com/tokio-rs/console/commit/a2202f76beb0cc7983355aec108697f8964fe837))
- Add information on where to put .cargo/config.toml ([#284](https://github.com/tokio-rs/console/issues/284)) ([d07aa89](https://github.com/tokio-rs/console/commit/d07aa89b168a120c47fb4bc88d6691a157406631))
- Document minimum Tokio versions ([#291](https://github.com/tokio-rs/console/issues/291)) ([3b1f14a](https://github.com/tokio-rs/console/commit/3b1f14a50c507e7b5b672491fada6dfb067fc671), closes [#281](https://github.com/tokio-rs/console/issues/281))

### Fixed

- Fix compilation on targets without 64-bit atomics ([#282](https://github.com/tokio-rs/console/issues/282)) ([5590fdb](https://github.com/tokio-rs/console/commit/5590fdbc3e7f78c6a3800f0e07c148320447788e), fixes [#279](https://github.com/tokio-rs/console/issues/279))
- Bail rather than panic when encountering clock skew ([#287](https://github.com/tokio-rs/console/issues/287)) ([24db8c6](https://github.com/tokio-rs/console/commit/24db8c603fc86199f54a074a08390c68d1aa04e1), fixes [#286](https://github.com/tokio-rs/console/issues/286))
- Use monotonic `Instant`s for all timestamps ([#288](https://github.com/tokio-rs/console/issues/288)) ([abc0830](https://github.com/tokio-rs/console/commit/abc083000cb6de51e37d5037283e97ed0e27249e), fixes [#286](https://github.com/tokio-rs/console/issues/286))
- Record timestamps for updates last ([#289](https://github.com/tokio-rs/console/issues/289)) ([703f1aa](https://github.com/tokio-rs/console/commit/703f1aa449c7579d15af8adfbfc172e75da99281), fixes [#266](https://github.com/tokio-rs/console/issues/266))

## console-subscriber-v0.1.2 - (2022-02-07)

[12a4821](https://github.com/tokio-rs/console/commit/12a4821a0058dd6e1a0e4f6729a0f78fac291e0e)...[e590df3](https://github.com/tokio-rs/console/commit/e590df39ca38cf795b1aec493403e1411e3b4766)


### Fixed

- Console-api dependencies to require 0.1.2 ([#274](https://github.com/tokio-rs/console/issues/274)) ([b95f683](https://github.com/tokio-rs/console/commit/b95f683f0514978429535a75c86f8974b05a69aa))

## console-subscriber-v0.1.1 - (2022-01-18)

[d3a410e](https://github.com/tokio-rs/console/commit/d3a410e5aaeb96fd061f47ae61fdadcce5b195d7)...[12a4821](https://github.com/tokio-rs/console/commit/12a4821a0058dd6e1a0e4f6729a0f78fac291e0e)


### Fixed

- Use saturating arithmetic for attribute updates ([#234](https://github.com/tokio-rs/console/issues/234)) ([fe82e17](https://github.com/tokio-rs/console/commit/fe82e1704686ccbcdabaa1715cf30c5ce15cc17a))
- Increase default event buffer capacity a bit ([#235](https://github.com/tokio-rs/console/issues/235)) ([0cf0aee](https://github.com/tokio-rs/console/commit/0cf0aee31af1cf6992e98db8269fbfcec2d54961))
- Only send *new* tasks/resources/etc over the event channel ([#238](https://github.com/tokio-rs/console/issues/238)) ([fdc77e2](https://github.com/tokio-rs/console/commit/fdc77e28d45da73595320fab8ce56f982c562bb6))

## console-subscriber-v0.1.0 - (2021-12-16)


### Added

- Assert `tokio-unstable` is on ([776966e](https://github.com/tokio-rs/console/commit/776966ea1444525490b9f060e96555809d44cf26))
- Send structured fields on the wire ([#26](https://github.com/tokio-rs/console/issues/26)) ([38adbd9](https://github.com/tokio-rs/console/commit/38adbd97aefc53d06e509c7b33c98b4dcfa7a970), fixes [#6](https://github.com/tokio-rs/console/issues/6))
- Drop data for completed tasks  ([#31](https://github.com/tokio-rs/console/issues/31)) ([94aad1c](https://github.com/tokio-rs/console/commit/94aad1c88e9f97e08ef513449e1399092187da21))
- Emit waker stats ([#44](https://github.com/tokio-rs/console/issues/44)) ([2d2716b](https://github.com/tokio-rs/console/commit/2d2716badf35e3c887c8ab8dfd6ab64a721c6cf5), closes [#42](https://github.com/tokio-rs/console/issues/42))
- Record and send poll times with HdrHistogram ([#47](https://github.com/tokio-rs/console/issues/47)) ([94e7834](https://github.com/tokio-rs/console/commit/94e7834db44c3b19c54ff16a22f1b0e6464be1a2), closes [#36](https://github.com/tokio-rs/console/issues/36))
- Correctly reflect busy and idle times ([#60](https://github.com/tokio-rs/console/issues/60)) ([e48f005](https://github.com/tokio-rs/console/commit/e48f005cf6ed88cac94465b7ba56ad05477fd9b6), fixes [#59](https://github.com/tokio-rs/console/issues/59))
- Support multiple task callsites ([#68](https://github.com/tokio-rs/console/issues/68)) ([6b835e7](https://github.com/tokio-rs/console/commit/6b835e765fb43e9cf0dafef97ff3edf9042b7da7))
- Use sequential `u64` task IDs ([#75](https://github.com/tokio-rs/console/issues/75)) ([c2c486e](https://github.com/tokio-rs/console/commit/c2c486ee9c792453db81786490bff52a031be9e9))
- Remove fmt layer from init ([#64](https://github.com/tokio-rs/console/issues/64)) ([778a8f1](https://github.com/tokio-rs/console/commit/778a8f1fd60c1b92628cef59b021abf3fb0449a4))
- Add ability to record events to a file ([#86](https://github.com/tokio-rs/console/issues/86)) ([4fc72c0](https://github.com/tokio-rs/console/commit/4fc72c011ae5552ac4bd97cb69354f4205e1107f), closes [#84](https://github.com/tokio-rs/console/issues/84))
- Implement more design ideas from #25 ([#91](https://github.com/tokio-rs/console/issues/91)) ([ef9eafa](https://github.com/tokio-rs/console/commit/ef9eafad1e54acd6221d644e26ae7c01ce2eaed9))
- Periodically shrink growable collections ([#94](https://github.com/tokio-rs/console/issues/94)) ([9f7d499](https://github.com/tokio-rs/console/commit/9f7d4998106427170458fb1737dbd5e7ae16c1a4))
- Remove trace event calls from the subscriber ([#95](https://github.com/tokio-rs/console/issues/95)) ([246fc45](https://github.com/tokio-rs/console/commit/246fc45a76d6afb2ee6537b2ee73004570ffcbc9), closes [#27](https://github.com/tokio-rs/console/issues/27))
- Accept durations with units ([#93](https://github.com/tokio-rs/console/issues/93)) ([e590f83](https://github.com/tokio-rs/console/commit/e590f8318cc4ab6346d67f4f4c98a8b4d47c1d58))
- Add pause and resume ([#78](https://github.com/tokio-rs/console/issues/78)) ([1738481](https://github.com/tokio-rs/console/commit/173848173207afffce06c04a2ebfaa834794c6b1), closes [#85](https://github.com/tokio-rs/console/issues/85))
- Spill callsites into hash set  ([#97](https://github.com/tokio-rs/console/issues/97)) ([5fe4437](https://github.com/tokio-rs/console/commit/5fe443768dc9a63de2f6e66cf711e6fc535e8678))
- Resource instrumentation ([#77](https://github.com/tokio-rs/console/issues/77)) ([f4a21ac](https://github.com/tokio-rs/console/commit/f4a21acb18935af8b256999e2380eb5fb7e17d72))
- Represent readiness as bool ([#103](https://github.com/tokio-rs/console/issues/103)) ([ba95a38](https://github.com/tokio-rs/console/commit/ba95a38251a92ac3988333ab04655fa59d404937))
- Emit and show self-wake counts for tasks ([#112](https://github.com/tokio-rs/console/issues/112)) ([4023ad3](https://github.com/tokio-rs/console/commit/4023ad3be3db3a600f9351f3cdd3d25b45b3d6cb), closes [#55](https://github.com/tokio-rs/console/issues/55))
- Look at event parents to determine resource id ([#114](https://github.com/tokio-rs/console/issues/114)) ([0685482](https://github.com/tokio-rs/console/commit/06854828198fe3ab996c39d7bd7eaa7e87cffcae))
- Name tasks spawned by the console subscriber ([#117](https://github.com/tokio-rs/console/issues/117)) ([05b9f5b](https://github.com/tokio-rs/console/commit/05b9f5bf2accba58da97a4b08d4ab500892465b7))
- Add `retain-for` cmd line arg ([#119](https://github.com/tokio-rs/console/issues/119)) ([7231a33](https://github.com/tokio-rs/console/commit/7231a33268d409e4188c49b91ae8fc77c2889df6))
- Use per-layer filtering ([#140](https://github.com/tokio-rs/console/issues/140)) ([f2c30d5](https://github.com/tokio-rs/console/commit/f2c30d52c9f22de69bac38009a9183135808806c), closes [#76](https://github.com/tokio-rs/console/issues/76))
- Use `Location` for tasks and resources ([#154](https://github.com/tokio-rs/console/issues/154)) ([08c5186](https://github.com/tokio-rs/console/commit/08c5186eb01f18f8e4018058d12817e4127dd7be))
- Enable spans with names starting with `runtime` ([#156](https://github.com/tokio-rs/console/issues/156)) ([3c50514](https://github.com/tokio-rs/console/commit/3c50514060724e0655d44b58f16fd282d84ce43b))
- Resources UI ([#145](https://github.com/tokio-rs/console/issues/145)) ([577fb55](https://github.com/tokio-rs/console/commit/577fb55e48de052b9cd186476f092c76317bc09f))
- Do not print errors when we cannot determine task context ([#186](https://github.com/tokio-rs/console/issues/186)) ([bdcdcb1](https://github.com/tokio-rs/console/commit/bdcdcb1675b80758c2177dfb5e71426957f02cee))
- Unify build, init, and the Layer system ([#195](https://github.com/tokio-rs/console/issues/195)) ([457f525](https://github.com/tokio-rs/console/commit/457f525fd59bc9683a6dda7fcdb2bc225ee2cf71), closes [#183](https://github.com/tokio-rs/console/issues/183), closes [#196](https://github.com/tokio-rs/console/issues/196))
- Add resource detail view ([#188](https://github.com/tokio-rs/console/issues/188)) ([1aa9b59](https://github.com/tokio-rs/console/commit/1aa9b594f30e42098c6c6bbf41eb1d2b01dc0426))
- Rename `TasksLayer` to `ConsoleLayer` ([#207](https://github.com/tokio-rs/console/issues/207)) ([fbadf2f](https://github.com/tokio-rs/console/commit/fbadf2fe795a822c0843789b3113d9c297883345))
- Count dropped events due to buffer cap ([#211](https://github.com/tokio-rs/console/issues/211)) ([aa09600](https://github.com/tokio-rs/console/commit/aa09600b3bdc6591eafc9fe7b4507f7da2bca498))

### Documented

- Add some misbehaving options to example app ([#54](https://github.com/tokio-rs/console/issues/54)) ([5568bf6](https://github.com/tokio-rs/console/commit/5568bf6cdfd22af57b5dd6d0ef283466ec77058c))
- Add Netlify auto-publishing of `main` API docs ([#116](https://github.com/tokio-rs/console/issues/116)) ([b0c5a9d](https://github.com/tokio-rs/console/commit/b0c5a9d269b571459395d7ef08b7c09f53adc39b))
- Add a README (and `lib.rs` docs) ([#202](https://github.com/tokio-rs/console/issues/202)) ([a79c505](https://github.com/tokio-rs/console/commit/a79c5056875a3593b4fd61d18e42c2aa6a08688c))
- Use H1 headers in builder API docs ([#204](https://github.com/tokio-rs/console/issues/204)) ([6261e15](https://github.com/tokio-rs/console/commit/6261e15b6b7e2eab3a235a8d7693ca61855d03e7))
- Console-subscriber API docs ([#198](https://github.com/tokio-rs/console/issues/198)) ([7d16ead](https://github.com/tokio-rs/console/commit/7d16eadc5c254f21b0f4fba31f2fdf758808a8f4))

### Fixed

- Fix busy loop in aggregator task ([#17](https://github.com/tokio-rs/console/issues/17)) ([fff4698](https://github.com/tokio-rs/console/commit/fff46989221f0eea53303abaf08e6e2f29476500))
- Use correct timestamps for Stats::to_proto ([#19](https://github.com/tokio-rs/console/issues/19)) ([90d38b1](https://github.com/tokio-rs/console/commit/90d38b169f61982f0158aa3ae4f23b039cd96102))
- Require tokio >= 1.5 ([#22](https://github.com/tokio-rs/console/issues/22)) ([62dec4a](https://github.com/tokio-rs/console/commit/62dec4af406df453924be1133cef2963c6979999))
- Update uncompleted tasks total time every update ([#28](https://github.com/tokio-rs/console/issues/28)) ([d7f1629](https://github.com/tokio-rs/console/commit/d7f16293d939886e1f16afb80fc92033473e6312))
- Detect completed tasks even if console connects after ([#29](https://github.com/tokio-rs/console/issues/29)) ([53515a7](https://github.com/tokio-rs/console/commit/53515a7d9532e8b9780b56ab95d067309b46dc6f))
- Consider by-value wakes to be waker drops ([#46](https://github.com/tokio-rs/console/issues/46)) ([aeaecf5](https://github.com/tokio-rs/console/commit/aeaecf5467c188acde1c14a18261eade864bcdb9))
- Enable `runtime::` target for tracing events ([#99](https://github.com/tokio-rs/console/issues/99)) ([0da7243](https://github.com/tokio-rs/console/commit/0da72436ee42a11ab32003efa1353b1de52691fb))
- Remove backticks from mangled PR review suggestion ([#105](https://github.com/tokio-rs/console/issues/105)) ([1ad57af](https://github.com/tokio-rs/console/commit/1ad57af03fd007a2357eb299e3c8f254dc10f302))
- Include tracing events starting with tokio in filter ([#159](https://github.com/tokio-rs/console/issues/159)) ([6786d3e](https://github.com/tokio-rs/console/commit/6786d3e86966ff0524a3ed855caeff864be12b15), closes [#149](https://github.com/tokio-rs/console/issues/149))
- Remove chrono from deps and sub-deps ([#175](https://github.com/tokio-rs/console/issues/175)) ([c4e3302](https://github.com/tokio-rs/console/commit/c4e3302a118e5da030686cdd8a68cb8c55629567))
- Unset default dispatcher in span callbacks ([#170](https://github.com/tokio-rs/console/issues/170)) ([3170432](https://github.com/tokio-rs/console/commit/31704326f2e8665a7f062ceca84bf8d843007017))
- Fix potential spurious flush notifications ([#178](https://github.com/tokio-rs/console/issues/178)) ([c5e9b37](https://github.com/tokio-rs/console/commit/c5e9b375540bdb9a08370fe5d305d77efe0a63a7))
- Ignore spans that weren't initially recorded ([0cd7a2f](https://github.com/tokio-rs/console/commit/0cd7a2f76bcac4c609771d20f4c0fb21f10b62d4))
- Ignore exiting spans that were never entered ([ad442e2](https://github.com/tokio-rs/console/commit/ad442e2852065b6c5d7a770d2ef68439945354c7))

<!-- generated by git-cliff -->
