# Changelog

## [0.2.0](https://github.com/Gerixmus/gitquick/compare/v0.1.13...v0.2.0) (2025-12-11)


### ⚠ BREAKING CHANGES

* due to the project rename the default config path needed to be changed
* config file now uses tables

### Features

* allow custom commit types ([#63](https://github.com/Gerixmus/gitquick/issues/63)) ([fc6b969](https://github.com/Gerixmus/gitquick/commit/fc6b96933da76c9c1244431d8e1061a158a7c065))
* **commit:** add body ([#56](https://github.com/Gerixmus/gitquick/issues/56)) ([0028930](https://github.com/Gerixmus/gitquick/commit/002893080055cc7cbf2f34914c2d6bd3853c52cf))
* implement add command ([#54](https://github.com/Gerixmus/gitquick/issues/54)) ([0a908c9](https://github.com/Gerixmus/gitquick/commit/0a908c9530522629372ab4a72c537badd39b0a69))
* read custom branch types from toml ([#65](https://github.com/Gerixmus/gitquick/issues/65)) ([9cb303a](https://github.com/Gerixmus/gitquick/commit/9cb303a3b2f517d5f5727889584ebb8512bff16f))
* restructure init command ([#46](https://github.com/Gerixmus/gitquick/issues/46)) ([a796926](https://github.com/Gerixmus/gitquick/commit/a7969268f29053b1588566c4117f6ef18225358a))


### Bug Fixes

* remove reset on branch display ([#48](https://github.com/Gerixmus/gitquick/issues/48)) ([b628823](https://github.com/Gerixmus/gitquick/commit/b6288231bbbc33012698fee7bccc330c96ebe2ee))
* use default values if not provided in toml ([#66](https://github.com/Gerixmus/gitquick/issues/66)) ([02134fb](https://github.com/Gerixmus/gitquick/commit/02134fb8822e5b2708003b3cc54848b782856246))


### Code Refactoring

* change config path to contain gitquick ([#64](https://github.com/Gerixmus/gitquick/issues/64)) ([eacf165](https://github.com/Gerixmus/gitquick/commit/eacf165786dc867444138f034ddbbf31bc3e6cda))

## [0.1.13](https://github.com/Gerixmus/gitquick/compare/v0.1.12...v0.1.13) (2025-08-13)


### Features

* **commit:** add breaking change support ([#43](https://github.com/Gerixmus/gitquick/issues/43)) ([73d581d](https://github.com/Gerixmus/gitquick/commit/73d581d5efd458ee429008167a0d647be0b87e8b))

## [0.1.12](https://github.com/Gerixmus/gitquick/compare/v0.1.11...v0.1.12) (2025-08-12)


### Features

* add scope ([#41](https://github.com/Gerixmus/gitquick/issues/41)) ([039fd24](https://github.com/Gerixmus/gitquick/commit/039fd24ec0326d8ab175debdfc8e606be3bc0bbf))

## [0.1.11](https://github.com/Gerixmus/gitquick/compare/v0.1.10...v0.1.11) (2025-08-11)


### Features

* push only if configured ([#39](https://github.com/Gerixmus/gitquick/issues/39)) ([4325e3a](https://github.com/Gerixmus/gitquick/commit/4325e3ad5585367dbc28ef224081e244401502db))

## [0.1.10](https://github.com/Gerixmus/gitquick/compare/v0.1.9...v0.1.10) (2025-08-07)


### Features

* add conventional branch flag ([#35](https://github.com/Gerixmus/gitquick/issues/35)) ([895a787](https://github.com/Gerixmus/gitquick/commit/895a78763f55bfb28b82b6b91fee86df04e66c71))

## [0.1.9](https://github.com/Gerixmus/gitquick/compare/v0.1.8...v0.1.9) (2025-08-07)


### Features

* add delete branch command ([#33](https://github.com/Gerixmus/gitquick/issues/33)) ([f524325](https://github.com/Gerixmus/gitquick/commit/f524325b61a4148104529dd46799862d3c7e0c97))

## [0.1.8](https://github.com/Gerixmus/gitquick/compare/v0.1.7...v0.1.8) (2025-08-06)


### Features

* replace spaces with hyphens ([#31](https://github.com/Gerixmus/gitquick/issues/31)) ([c37b5a5](https://github.com/Gerixmus/gitquick/commit/c37b5a58480d4663b8ab843ff6497a7fa64f05b3))

## [0.1.7](https://github.com/Gerixmus/gitquick/compare/v0.1.6...v0.1.7) (2025-08-05)


### Bug Fixes

* add recurse untracked dirs option ([#29](https://github.com/Gerixmus/gitquick/issues/29)) ([c91bd51](https://github.com/Gerixmus/gitquick/commit/c91bd51ab73a06bffc343661982c2c493c184efc))

## [0.1.6](https://github.com/Gerixmus/gitquick/compare/v0.1.5...v0.1.6) (2025-08-04)


### Features

* remove ssh functionality ([#25](https://github.com/Gerixmus/gitquick/issues/25)) ([f660eac](https://github.com/Gerixmus/gitquick/commit/f660eacf1aae898a3790fc84d84e38e7fbca3c6f))

## [0.1.5](https://github.com/Gerixmus/gitquick/compare/v0.1.4...v0.1.5) (2025-07-10)


### Bug Fixes

* message for http push ([#22](https://github.com/Gerixmus/gitquick/issues/22)) ([e5d38ab](https://github.com/Gerixmus/gitquick/commit/e5d38ab90d13496b6a07119d0e86f2eb781b5f55))

## [0.1.4](https://github.com/Gerixmus/gitquick/compare/v0.1.3...v0.1.4) (2025-07-05)


### Bug Fixes

* ssh agent callback missing ([#20](https://github.com/Gerixmus/gitquick/issues/20)) ([e7fee7c](https://github.com/Gerixmus/gitquick/commit/e7fee7c94a1492a78674b2cd0b9954f297ad3856))

## [0.1.3](https://github.com/Gerixmus/gitquick/compare/v0.1.2...v0.1.3) (2025-07-05)


### Bug Fixes

* correct argument for branch command ([#17](https://github.com/Gerixmus/gitquick/issues/17)) ([bb899c7](https://github.com/Gerixmus/gitquick/commit/bb899c71d949ef3307bf7e742b1ce0ff03181b3d))

## [0.1.2](https://github.com/Gerixmus/gitquick/compare/v0.1.1...v0.1.2) (2025-07-05)


### Features

* create branch and checkout ([#15](https://github.com/Gerixmus/gitquick/issues/15)) ([e5b7159](https://github.com/Gerixmus/gitquick/commit/e5b7159512e7cfb4ae5605f5ca28f3f8b1f5b890))

## [0.1.1](https://github.com/Gerixmus/gitquick/compare/v0.1.0...v0.1.1) (2025-07-05)


### Features

* added commit functionality ([78f336e](https://github.com/Gerixmus/gitquick/commit/78f336e418504e8e58d5b8319942e0e83597f86f))
* added commit subcommand ([6f4e550](https://github.com/Gerixmus/gitquick/commit/6f4e550ed2dacf1016dfa9c362e93a4d8c222ccb))
* added config ([bddc774](https://github.com/Gerixmus/gitquick/commit/bddc77441a9befd8c6623018517f70e5214be028))
* added logic for showing branches ([937671e](https://github.com/Gerixmus/gitquick/commit/937671ed51ac75174fdd3cc4772a78afb142955f))
* added prompt for verification ([ab15c0e](https://github.com/Gerixmus/gitquick/commit/ab15c0e12de92628cca7e7bd1289d54cd38fd18a))
* added simple ci ([562d0c2](https://github.com/Gerixmus/gitquick/commit/562d0c2b92f57d2140547b652349105ff1709725))
* added subcommand ([29f511f](https://github.com/Gerixmus/gitquick/commit/29f511f14414db6007250d4a9e26f406cc29e45b))
* automatic push ([e4aff89](https://github.com/Gerixmus/gitquick/commit/e4aff89152c8c7e35f01aba8eba8dc03f19c77b5))
* command for config setup ([2168198](https://github.com/Gerixmus/gitquick/commit/2168198c621ffdccd9a9b97b47134170ff4dd562))
* created fn for untracked files ([2ff601b](https://github.com/Gerixmus/gitquick/commit/2ff601b32a7973476f57e437009c95190e450815))
* deletions visible by commit command ([f662d29](https://github.com/Gerixmus/gitquick/commit/f662d291808111b457c227a8addd5de6c987b7e6))
* logic for checkout and branch list ([e2c5185](https://github.com/Gerixmus/gitquick/commit/e2c5185bfe0b45d8b8e4b638013386a600cc049b))
* logic for checkout and branch list ([df42472](https://github.com/Gerixmus/gitquick/commit/df424725e21cb0590433ecc82631904e499bda62))
* multi select demo ([43f5d1e](https://github.com/Gerixmus/gitquick/commit/43f5d1ec8265315f964b946eb34c1b9f63d1ffb5))
* new command ([88bf345](https://github.com/Gerixmus/gitquick/commit/88bf345c85d9426106977054c8f4337180bc8bc1))
* push using git2 package ([#6](https://github.com/Gerixmus/gitquick/issues/6)) ([99f1b65](https://github.com/Gerixmus/gitquick/commit/99f1b659b783a85d6a7bf439bf46bd253a4a5cf3))
* release step in pipeline ([a7f03ec](https://github.com/Gerixmus/gitquick/commit/a7f03ec71bcf83a2b1863163b881254fa4ae0f46))
* retrieval of untracked files ([dfb8d68](https://github.com/Gerixmus/gitquick/commit/dfb8d6814ba754560b6e69683e45124308330a76))
* retrieving config file from platform specific location ([687f233](https://github.com/Gerixmus/gitquick/commit/687f233a1b62f190df3d1f4e241202d8a90cc14d))
* rust project created ([5d19e62](https://github.com/Gerixmus/gitquick/commit/5d19e629d811da7058e77bef0b9fa4dc24c123cd))
* selected files being staged ([25353ea](https://github.com/Gerixmus/gitquick/commit/25353ea5a16e555d22e19c252941d38b36056566))
* visible upstream info ([eafe464](https://github.com/Gerixmus/gitquick/commit/eafe464b86e69e3487397e014cb7b03c77e8496b))


### Bug Fixes

* add only after confirmation ([ffcb6ee](https://github.com/Gerixmus/gitquick/commit/ffcb6ee22a5e6e4573ef83265c980890efa6c850))
* changed package name ([659eb44](https://github.com/Gerixmus/gitquick/commit/659eb44dea072dc73175c23d336e46a04879e9aa))
* correct message printed ([7ad2eb9](https://github.com/Gerixmus/gitquick/commit/7ad2eb91fc9482898051a99b4242e6efeb161576))
* missing directories created on config ([2f59309](https://github.com/Gerixmus/gitquick/commit/2f5930925ff4f5ad4fa726e2cfe29f7131a68e43))
* missing permission in publish action ([ebc9142](https://github.com/Gerixmus/gitquick/commit/ebc91421f4cd3b2ce5619a639a1620214a091b1a))
* missing permissions ([f39fc3b](https://github.com/Gerixmus/gitquick/commit/f39fc3b50f67b9987fd5d7795c32455102c0065b))
* missing success message ([881f44a](https://github.com/Gerixmus/gitquick/commit/881f44a1d65f6393f1a96ca01040d76718e8747e))
* path check used for deletion ([5a40fe4](https://github.com/Gerixmus/gitquick/commit/5a40fe46baf4f365a0181be741b469c649e73a24))
* running fetch prune before checking branches ([5c6e2b0](https://github.com/Gerixmus/gitquick/commit/5c6e2b06df6592d69946bd50923181f25375b4c7))
* staged files retrieved by commit command ([464447f](https://github.com/Gerixmus/gitquick/commit/464447f7bef63a05b209a98317fca17c313b2177))
* wrong format for tags ([90507c5](https://github.com/Gerixmus/gitquick/commit/90507c508041963755205fd898af0ed2a7edf5f1))
