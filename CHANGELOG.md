# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning][semver].

## [0.6.1] - 2021-02-01

### Added

- [dependency] Add features to depend on tokio `0.2`. \[[@AriusX7]; [c:d075c5a]]

### Changes

- Ignore errors on reactions cleanup in `Menu`. \[[@AriusX7]; [c:f0949ea]]

## [0.6.0] - 2021-01-10

### Changed

- [dependency] Bump to serenity `0.10`. \[[@AriusX7]; [c:382b114]]
- [dependency] Bump to tokio `1.0`. \[[@AriusX7]; [c:382b114]]
- [dependency] Update examples to serenity `0.10` and tokio `1.0`. \[[@AriusX7]; [c:00c1b0a]]

## [0.5.1] - 2020-11-28

### Changed

- Fix `Conversion` implementations so `cache` feature can be omitted without errors. \[[@Headline]; [c:53db2ae]]
- [dependency] Bumped to serenity `0.9.1`. \[[@AriusX7]; [c:99f35a6]]
- [documentation] Change to use intra-doc links. \[[@AriusX7]; [c:2d43851]]
- [meta] Change directory structure to be consistent with Rust 2018 idioms. \[[@AriusX7]; [c:41a3b91]]

## [0.5.0] - 2020-08-31

### Changed

- Convert `Error` into an enum. \[[@AriusX7]]
- [documentation] Add info about return error types. \[[@AriusX7]]

## [0.4.0] - 2020-08-28

### Added

- `misc` module with `add_reactions` and `add_reactions_blocking` functions. \[[@AriusX7]]
- `non_blocking` field for `MenuOptions`. \[[@AriusX7]]
- [dependency] Add tokio. \[[@AriusX7]]
- Add tests. \[[@AriusX7]]

### Changed

- Make `reaction_prompt` add reactions in a separate, non-blocking task. \[[@AriusX7]]
- Convert `examples` into a workspace. \[[@AriusX7]]
- Allow `Menu` to have reactions added in a non-blocking fashion. \[[@AriusX7]]

## [0.3.0] - 2020-08-25

### Changed

- Change `Menu` to use `CreateMessage`. \[[@AriusX7]]
- Change `Menu::run` to take ownership instead of reference. \[[@AriusX7]]
- Reduce unnecessary clones in `Menu` methods. \[[@AriusX7]]
- Change `msg.react()` with `http.create_reaction()` to avoid cloning `emoji`. \[[@AriusX7]]

## [0.2.0] - 2020-08-23

### Added

- Add message and embed builders. \[[@AriusX7]]
- Add full message support for menus. \[[@AriusX7]]
- Preludes to easily import commonly used types at once. \[[@AriusX7]]
- [documentation] Add CHANGELOG.md \[[@AriusX7]]

### Changed

- Change `Menu` to use `MessageBuilder`. \[[@AriusX7]]

[semver]: https://semver.org/spec/v2.0.0.html

<!-- TAGS -->
[0.2.0]: https://github.com/AriusX7/serenity-utils/compare/v0.1.0...v0.2.0
[0.3.0]: https://github.com/AriusX7/serenity-utils/compare/v0.2.0...v0.3.0
[0.4.0]: https://github.com/AriusX7/serenity-utils/compare/v0.3.0...v0.4.0
[0.5.0]: https://github.com/AriusX7/serenity-utils/compare/v0.4.0...v0.5.0
[0.5.1]: https://github.com/AriusX7/serenity-utils/compare/v0.5.0...v0.5.1
[0.6.0]: https://github.com/AriusX7/serenity-utils/compare/v0.5.1...v0.6.0
[0.6.1]: https://github.com/AriusX7/serenity-utils/compare/v0.6.0...v0.6.1

<!-- CONTRIBUTORS -->
[@AriusX7]: https://github.com/AriusX7
[@Headline]: https://github.com/Headline

<!-- COMMITS -->
[c:f0949ea]: https://github.com/AriusX7/serenity-utils/commit/f0949eae2f13b43146989c246754e1b3137506b6
[c:d075c5a]: https://github.com/AriusX7/serenity-utils/commit/d075c5afff8bd31a88ee783fc53f650bce073bdc
[c:00c1b0a]: https://github.com/AriusX7/serenity-utils/commit/00c1b0a2df9be211b2c880ae5ade0ec8d17a601b
[c:382b114]: https://github.com/AriusX7/serenity-utils/commit/382b1143abf931ac0efff93404523d65d0741bfa
[c:99f35a6]: https://github.com/AriusX7/serenity-utils/commit/99f35a6f502302b7242a13fa0e11bc5eb7adc460
[c:41a3b91]: https://github.com/AriusX7/serenity-utils/commit/41a3b91536368719a1f7dcc4f217808414acf770
[c:2d43851]: https://github.com/AriusX7/serenity-utils/commit/2d4385195826027a486e4b1752a2ceac17fb3b99
[c:53db2ae]: https://github.com/AriusX7/serenity-utils/commit/53db2aef3673b6fff4c49c2a787c17f7d8da0cb7
