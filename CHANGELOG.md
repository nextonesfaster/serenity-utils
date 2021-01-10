# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning][semver].

## [0.5.2] - 2020-11-28

### Changed

- [dependency] Bump to serenity `0.10`. \[[@AriusX7]; [c:d12ab9f]]
- [dependency] Bump to tokio `1.0`. \[[@AriusX7]; [c:d12ab9f]]
- [dependency] Update examples to serenity `0.10` and tokio `1.0`. \[[@AriusX7]; [c:47f1e6a]]

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
[0.5.2]: https://github.com/AriusX7/serenity-utils/compare/v0.5.1...v0.5.2

<!-- CONTRIBUTORS -->
[@AriusX7]: https://github.com/AriusX7
[@Headline]: https://github.com/Headline

<!-- COMMITS -->
[c:47f1e6a]: https://github.com/AriusX7/serenity-utils/commit/47f1e6acaaa90f4e279bdcdd16fbf136fc8a27ef
[c:d12ab9f]: https://github.com/AriusX7/serenity-utils/commit/d12ab9fc95fa2ebce431c4682c78e2f8eb21a836
[c:99f35a6]: https://github.com/AriusX7/serenity-utils/commit/99f35a6f502302b7242a13fa0e11bc5eb7adc460
[c:41a3b91]: https://github.com/AriusX7/serenity-utils/commit/41a3b91536368719a1f7dcc4f217808414acf770
[c:2d43851]: https://github.com/AriusX7/serenity-utils/commit/2d4385195826027a486e4b1752a2ceac17fb3b99
[c:53db2ae]: https://github.com/AriusX7/serenity-utils/commit/53db2aef3673b6fff4c49c2a787c17f7d8da0cb7
