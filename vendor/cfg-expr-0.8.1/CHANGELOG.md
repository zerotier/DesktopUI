# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
## [0.8.1] - 2021-08-05
### Changed
- [PR#31](https://blog.rust-lang.org/2021/05/06/Rust-1.52.0.html) reverted the usage of "or patterns" that were only added in 1.53.0. We now state the MSRV as 1.52.0. Thanks [@cgwalters](https://github.com/cgwalters)!

## [0.8.0] - 2021-07-16
### Changed
- [PR#28](https://github.com/EmbarkStudios/cfg-expr/pull/28) updated target-lexicon to 0.12. Thanks [@remilauzier](https://github.com/remilauzier)!
- [PR#29](https://github.com/EmbarkStudios/cfg-expr/pull/29) updated the built-in target list to 1.53.0.

## [0.7.4] - 2021-03-16
### Added
- [PR#26](https://github.com/EmbarkStudios/cfg-expr/pull/26) added `Expression::original` to get the original string the expression was parsed from. Thanks [@gdesmott](https://github.com/gdesmott)!

## [0.7.3] - 2021-03-16
### Added
- [PR#25](https://github.com/EmbarkStudios/cfg-expr/pull/25) added `Clone` for `Expression`. Thanks [@gdesmott](https://github.com/gdesmott)!
## [0.7.2] - 2021-03-16
### Added
- [PR#23](https://github.com/EmbarkStudios/cfg-expr/pull/23) added a `PartialEq` implementation for `Expression`, primarily for cases where an `Expression` is stored in a type that itself requires `PartialEq`. This is only a simple syntactical equality check. Thanks [@gdesmott](https://github.com/gdesmott)!

## [0.7.1] - 2021-02-17
### Fixed
- Fixed support for the `uclibceabi` environment added for one target in rust 1.50.0.

## [0.7.0] - 2021-02-12
### Changed
- Updated the builtin target list to Rust 1.50.0. Again, somewhat of a breaking change as many targets were removed or changed.

### Fixed
- Update smallvec to fix an [advisory](https://rustsec.org/advisories/RUSTSEC-2021-0003)

## [0.6.0] - 2021-01-04
### Changed
- Updated the builtin target list to Rust 1.49.0, this is somewhat of a breaking change, as rustc now considers all `android` targets to have the `gnu` environment, where previously, it was unspecified.

## [0.5.1] - 2020-12-15
### Changed
- Updated the builtin target list to Rust 1.48.0

## [0.5.0] - 2020-10-20
### Changed
- Updated the builtin target list to Rust 1.47.0

## [0.4.1] - 2020-06-04
### Fixed
- Removed `dbg!` prints accidentally left in.

## [0.4.0] - 2020-06-04
### Added
- [PR#9](https://github.com/EmbarkStudios/cfg-expr/pull/9) added the optional `targets` feature, which allows matching the various `target_` predicates against a [`target_lexicon::Triple`](https://docs.rs/target-lexicon/0.11.0/target_lexicon/struct.Triple.html).

### Changed
- [PR#9](https://github.com/EmbarkStudios/cfg-expr/pull/9) changed the `Arch`, `Vendor`, `Os`, and `Env` types to no be longer enums, and are instead thin wrappers around strings. This allows for custom targets where one or more components of the target triple are not built-in to rustc. Resolved [#8](https://github.com/EmbarkStudios/cfg-expr/issues/8).
- Changed `ParseError` to remove the lifetime and just keep an owned string of the expression that failed to parse.
- Updated the list of built-in rustc targets to 1.43.1.

## [0.3.0] - 2020-04-05
### Changed
- [PR#7](https://github.com/EmbarkStudios/cfg-expr/pull/7) changed `Expression::eval` to take a `Logic` trait, to enable evaluation of 'unknown' predicates. Thanks [@sunshowers](https://github.com/sunshowers)!

## [0.2.1] - 2020-03-30
### Fixed
- [PR#6](https://github.com/EmbarkStudios/cfg-expr/pull/6) fixed nested predicate evalution. Thanks [@sunshowers](https://github.com/sunshowers)!

## [0.2.0] - 2020-02-05
### Added
- Added `targets::rustc_version` which can be used to retrieve the version string of the the rustc used to generate the list of targets.

### Changed
- `targets::ALL` now uses the built-in targets for rustc 1.41.0

## [0.1.0] - 2020-01-09
### Added
- Initial add of all the things

<!-- next-url -->
[Unreleased]: https://github.com/EmbarkStudios/cfg-expr/compare/0.8.1...HEAD
[0.8.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.8.0...0.8.1
[0.8.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.7.4...0.8.0
[0.7.4]: https://github.com/EmbarkStudios/cfg-expr/compare/0.7.3...0.7.4
[0.7.3]: https://github.com/EmbarkStudios/cfg-expr/compare/0.7.2...0.7.3
[0.7.2]: https://github.com/EmbarkStudios/cfg-expr/compare/0.7.1...0.7.2
[0.7.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.7.0...0.7.1
[0.7.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.5.1...0.6.0
[0.5.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.4.1...0.5.0
[0.4.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/EmbarkStudios/cfg-expr/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/EmbarkStudios/cfg-expr/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/EmbarkStudios/cfg-expr/releases/tag/0.1.0
