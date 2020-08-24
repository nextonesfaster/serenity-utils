# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning][semver].

## [0.3.0] - 2020-08-25

### Changed

- Change `Menu` to use `CreateMessage`.
- Change `Menu::run` to take ownership instead of reference.
- Reduce unnecessary clones in `Menu` methods.
- Change `msg.react()` with `http.create_reaction()` to avoid cloning `emoji`.

## [0.2.0] - 2020-08-23

### Added

- Add message and embed builders.
- Add full message support for menus.
- Preludes to easily import commonly used types at once.
- [documentation] Add CHANGELOG.md

### Changed

- Change `Menu` to use `MessageBuilder`.

[semver]: https://semver.org/spec/v2.0.0.html

<!-- TAGS -->
[0.2.0]: https://github.com/AriusX7/serenity-utils/compare/v0.1.0...v0.2.0
[0.2.0]: https://github.com/AriusX7/serenity-utils/compare/v0.2.0...v0.3.0
