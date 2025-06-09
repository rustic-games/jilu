# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning]. The file is auto-generated using [Conventional Commits].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0/

## Overview

- [unreleased](#unreleased)
- [`0.8.0`](#0.8.0) – _2025.06.09_
- [`0.7.0`](#0.7.0) – _2025.06.09_
- [`0.6.0`](#0.6.0) – _2025.06.09_
- [`0.5.0`](#0.5.0) – _2025.06.07_
- [`0.4.0`](#0.4.0) – _2020.08.16_
- [`0.3.0`](#0.3.0) – _2020.03.14_
- [`0.2.0`](#0.2.0) – _2019.09.17_
- [`0.1.1`](#0.1.1) – _2019.08.12_
- [`0.1.0`](#0.1.0) – _2019.08.12_

## _[Unreleased]_

_nothing new to show for… yet!_

<a id="0.8.0" />

## [0.8.0] – _Commit or don't, there is no such thing as a free lunch._

_2025.06.09_

Swap `--tag` for `--commit`, bringing us closer to a proper release
workflow.


### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Jean Mertz (<git@jeanmertz.com>)

### Changes

#### Features

- **Add commit functionality to release workflow** ([`83e7793`])

  The release workflow now supports committing changes in addition to
  creating tags. The `--tag` flag has been renamed to `--commit` to better
  reflect its expanded functionality. When used, Jilu will stage the
  changelog file, create a commit with the message "chore: Release
  v{version}", and then create the corresponding Git tag.

  This makes the tool useful in a variety of release workflows.

<a id="0.7.0" />

## [0.7.0] – _Tag, you're it!_

_2025.06.09_

Tag your releases, plain and simple.


### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Jean Mertz (<git@jeanmertz.com>)

### Changes

#### Features

- **Add `--tag` flag to create Git tags for releases** ([`95f91a0`])

  This change introduces a new `--tag` (`-t`, `RELEASE_TAG`) option that
  instructs Jilu to create Git tags for the latest release in the
  changelog. When used, the tool validates that there are no unreleased
  changes and creates a Git tag using the latest release's version and
  metadata.

<a id="0.6.0" />

## [0.6.0] – _The Goodies Release_

_2025.06.09_

A list of changes that make using Jilu a little easier.

- Configure Jilu using CLI arguments (see `jilu --help`).
- `sponge` no longer needed, let Jilu write the release notes to a file.
- Group unreleased commits into a release before a tag is created.
- Skip thanking specific contributors in the default template using
  `IGNORE_CONTRIBUTORS`.
- Support HTML comments in change log templates.


### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Jean Mertz (<git@jeanmertz.com>)

### Changes

#### Features

- **allow configuring Jilu using CLI arguments** ([`a33ac8e`])

  With this change, all existing environment variable-based configuration
  can not be configured using CLI arguments.

  The following arguments are available:

  - `CHANGELOG` (`CHANGELOG`)
    Path to the change log file. Defaults to `CHANGELOG.md`.

  - `--write` (`-w`, `WRITE_CHANGELOG`)
    Write to `CHANGELOG` file instead of printing it to `stdout`.

  - `--release` (`-r`, `RELEASE`)
    Version to use for the unreleased changes.

  - `--notes` (`-n`, `RELEASE_NOTES`)
    Release notes for the unreleased changes.

  - `--edit` (`-e`, `RELEASE_EDIT`)
    Edit the release notes in `$EDITOR`.

  - `--help` (`-h`)
    Print usage information.

  All arguments are optional. If no arguments or environment variables are
  set, Jilu uses `CHANGELOG.md` to read the inline configurations, if
  available, and prints to stdout.

  ```sh
  jilu --help

- **Optionally write change log to file** ([`56ce427`])

  Previously, `jilu` would always write the change log to stdout, allowing
  you to pipe it to a file if needed. This caused a few issues:

  - You needed to use `sponge` to keep the inline configuration in the
    relevant change log file.
  - Opening `$EDITOR` does not work reliably when piping stdout.

  With this change, those issues are resolved.

  The previous workflow was:

  ```sh
  jilu | sponge CHANGELOG.md
  ```

  The new is:

  ```sh
  jilu --write CHANGELOG.md
  ```

  - You provide the change log file as the last argument to `jilu`.
  - You (optionally) provide the `--write` flag to let `jilu` write the
    changes to the file instead of printing them to stdout.

  This removes the need for `sponge`, and allows the editor to be used
  when `RELEASE_EDIT` is set.

  You can still run `jilu` without any arguments, which reads
  `CHANGELOG.md` for any custom configurations, if it exists, and prints
  the result to stdout.

- **Show unreleased changes in release notes editor** ([`1d15fe8`])

  When creating a release with the `RELEASE` environment variable, if
  `RELEASE_EDIT` is set, the unreleased commit messages are now displayed
  in the release notes editor as comments. This provides immediate context
  about what changes are being included in the release, making it more
  convenient to write release notes without having to reference the git
  log separately.

- **Support grouping unreleased commits into release** ([`83192b5`])

  Jilu can now generate changelogs with unreleased commits grouped into a
  specified release version using the `RELEASE=<version>` environment
  variables, despite a non-existing Git tag. Set `RELEASE_NOTES=<notes>`
  to provide release notes, and `RELEASE_EDIT=true` to interactively edit
  release notes using the configured Git editor.

  This allows pre-generating the change log before tagging a new release.

- **support ignoring contributors in default template** ([`8c71b99`])

  The default change log template now supports filtering contributors
  through the `IGNORE_CONTRIBUTORS` environment variable. Contributors
  whose email addresses are listed in this comma-separated variable will
  be excluded from the "Contributions" section of the release notes.

  This allows maintainers to exclude automated or bot contributors
  from public acknowledgments while still preserving their commit
  history in the changeset.

#### Bug Fixes

- **Handle nested HTML comments in metadata parsing** ([`8b86c59`])

  The metadata parsing logic was incorrectly breaking on the first `<!--`
  encountered, causing issues when processing nested HTML comments.

  With this fix, HTML comments inside template configurations are now
  parsed correctly.

<a id="0.5.0" />

## [0.5.0] – _Winter has passed_

_2025.06.07_

After a long hiatus, the project is back to life.

This release brings a couple of quality of life improvements such as
support for custom change log file names, out-of-the-box remote URL
links, and Github merge commit formatting.

While the future is uncertain, I do expect to continue to work on this
project for at least a while, as I have a couple of other projects that
use, or will use Jilu, and with that will likely come the need for a few
more features.

Enjoy!


### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Jean Mertz (<git@jeanmertz.com>)

### Changes

#### Bug Fixes

- **Github-independent anchor links for releases** ([`6cc1f0f`])

  Before this commit, we relied on Github's automatic anchor links to
  allow clicking on releases in the "overview" section of the change log.

  Even on Github however, those links were broken for titled releases, as
  the anchor links would include the sanitized title of the release.

  This commit addresses both issues; the anchor links are now Github-
  independent, and the anchor links now work for titled and untitled
  releases.

#### Features

- **Support custom change log file via `CHANGELOG` env var** ([`11ae252`])

  Users can now specify a different file name for their change log by
  setting the `CHANGELOG` environment variable, providing flexibility for
  projects that use change log file names like `CHANGELOG.txt` or
  `HISTORY.md`.

- **Automatic PR linking for GitHub merge commits** ([`b598d90`])

  Jilu now automatically detects GitHub merge commit descriptions and
  extracts pull request numbers to generate clickable PR links in the
  changelog. When a commit description follows the pattern "<description>
  (#123)", the changelog will now linkify the `#123` part of the
  description.

  If the description does not match the described pattern, the old
  behavior is preserved: the description is rendered as-is, and no PR link
  is generated.

- **try to get default remote origin url** ([`a318e04`])

  With this commit, we try to get the default remote origin URL from Git's
  `origin` remote.

  If found, we use it to set the `github.repo` field in the configuration.

  This ensures a more useful default usage of the tool, as it will
  generate correct links when running `jilu` without any custom
  configuration in a repository with a remote named `origin`.

<a id="0.4.0" />

## [0.4.0] – _Commit scope templating_

_2020.08.16_

You can use the `scope` template function to print the scope of a
commit.

A `typeheader` filter exists to use custom scope names. Use the
`scope_headers` config flag as such:

```markdown
<!--
Config(
  scope_headers: {
      "ui": "User Interface",
  }
)
-->
```

This feature is similar to the already existing `type` function.

See the [default template file](./template.md) for an example of the
`type` feature, which should give you an idea on how to use `scope` and
its configurables.


### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Jan Christian Grünhage (<jan.christian@gruenhage.xyz>)

### Changes

#### Features

- **provide scopeheader filter** ([`99695ca`])

  This filter is analogous to the typeheader filter, it maps from a scope
  to a scope name suitable for changelog headers, with the map
  configurable from the config section.

- **expose commit scope during templating** ([`65bab03`])

<a id="0.3.0" />

## [0.3.0] – _More templating functionality_

_2020.03.14_

The [Tera][0] dependency was upgraded from `1.0.0-beta.15` to `1.1.0`,
which brings with it a [list of new features][1] to use in the changelog
template.

Additionally, a bug causing git tags to be incorrectly sorted got
squashed.

[0]: https://tera.netlify.com/
[1]: https://github.com/Keats/tera/blob/1a0ce70af178a5cb519a231cc6afeab947f1728e/CHANGELOG.md


### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Jean Mertz (<git@jeanmertz.com>)

### Changes

#### Bug Fixes

- **sort list of git tags** ([`a4bbbdc`])

#### Features

- **update Tera dependency to v1.1.0 stable** ([`f2c9f38`])

  This allows for more functionality to be used while templating the
  CHANGELOG.

  See the [Terra changelog][0] for more details.

  [0]: https://github.com/Keats/tera/blob/1a0ce70af178a5cb519a231cc6afeab947f1728e/CHANGELOG.md

<a id="0.2.0" />

## [0.2.0] – _Final release with required release title_

_2019.09.17_

_Because coming up with a release title for every new release is hard,
so the next release will have a way to not have a release title, but
still have release notes._

On to the release itself.

This is mostly a release full of fixes as reported by @mmstick, @Calmynt
and @kondanta. Thank you all!

Aside from a slew of bug fixes, the biggest new feature is support for
tags starting with a `v`, so both `0.1.0` and `v0.2.0` tags are now
recognised as release tags.

Enjoy!


### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Jean Mertz (<jean@mertz.fm>)

### Changes

#### Bug Fixes

- **assign commits to correct release** ([`b101f8c`])

  The algorithm to determine if a commit belongs to a specific release is
  incorrectly assigning commits to the wrong releases for any release
  except the first one.

  It iterates over all commits with an enumerator attached, then skip any
  commits belonging to previous releases, then skipping the commits
  belonging to the current release, and finally uses the new enumerator
  value as the count for the number of commits belonging to the release.

  This is incorrect, as the enumerator shouldn't start counting until
  after skipping the commits not belonging to the current release.

  This is fixed by simplifying the iterator logic to count the relevant
  commits.

- **add newline between release date and notes** ([`e7e88b7`])

- **ignore non-conventional commits** ([`ceabf81`])

  The documentation states that non-conventional commits are ignored in
  the final release notes, but this was not actually the case. It is now.

  The "other" commits are ignored by Jilu and won't show up in the change
  log.

- **remove extra whitespace at end of commit** ([`7ffd865`])

  The git2 library adds a newline at the end of commit messages, even if
  the message is a single line. This makes sense when printing it to the
  screen, but not for our parser.

  Our parser either accepts a single line commit:

      feat: my commit message

  Or a multi-line commit with a blank line between the subject and the
  body:

      feat: my commit message

      with a commit body!

  In the first case, git2 adds a newline after the subject, which falls
  between the first and second case, and is thus considered invalid.

  The solution is to always trim any extra whitespace at the end of the
  commit message.

- **split contributors per line** ([`3948e9c`])

  The default template didn't add a newline after each contributor,
  breaking the rendered markdown.

- **deduplicate list of contributors** ([`adbabcf`])

#### Features

- **better tag support** ([`6b79f87`])

  Tags can now start with or without a leading `v` (e.g. v0.1.0).

  Also, tags can now be both lightweight or annotated, whereas before
  non-annotated tags returned an error.

  When using non-annotated tags, a release won't have a title or a custom
  release description.

- **update to new conventional commit parser** ([`9ca8143`])

  The new parser uses the [Nom] library for improved accuracy and zero
  allocations with fewer dependencies.

  [nom]: https://docs.rs/nom

<a id="0.1.1" />

## [0.1.1] – _The Quick Fix_

_2019.08.12_

_What goes up, must come down._

This release fixes several issues that came to light after releasing
`v0.1.0`, which put the repository in a state with no unreleased
changes, triggering branching logic that still had a few bugs 🐛.

Those bugs are no more.

Still, this is a perfect reminder to start working on [those unit
tests].

[those unit tests]: https://github.com/rustic-games/jilu/issues/4


### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Jean Mertz (<jean@mertz.fm>)

### Changes

#### Bug Fixes

- **prevent subtraction with overflow** ([`2e383d1`])

  Return early if there are no commits to take from the stack.

- **correctly check for no new changes** ([`61ceeed`])

  Since `unreleased` is an object, it never reports back as being _falsy_,
  so we instead check for an empty list of changes in the object.

<a id="0.1.0" />

## [0.1.0] – _Ship It!_

_2019.08.12_

The first release of **Jilu** 🎉.

**Jilu** is a tool that generate a change log for you, based on the
state of your Git repository. It converts [conventional commits] into a
**human readable** change log, using Git tags to **annotate your
releases** with titles, release notes and more. You can **tweak your
change log** to suit the needs of your community and even integrate the
`jilu` binary into your CI workflow for **automated updates**.

This release is an example of using an [annotated Git tag] to attach a
custom release title (in this case "_Ship It!_") and a hand-written
release note (this message) to a release. This makes it more pleasant
for your readers to get up-to-date on what has changed, while also
providing them with an accurate list of all the relevant changes part of
the release (which for this project means all commits with the types
"feat", "fix" or "perf").

Since the notes are inlined into the change log, you can use markdown
and have it render as expected. Don't go _too_ crazy with this though,
as people might not always read your tag annotations from a client that
can render Markdown text to HTML. And while that is the exact purpose of
markdown (being easy to read in non-rendered form), you can still get
too carried away, making your notes less readable than they could be.

You can also embed images to give more visual appeal to your release
notes, as _a picture is worth a thousand words_ when you want to let
your audience know about all those amazing new features.

![Release Notes](https://user-images.githubusercontent.com/383250/62890397-b5ad4200-bd43-11e9-8043-8a096c737c1c.png?sanitize=true)

Now first I'm going to _automatically_ (really! 🙈) thank _myself_ for
my contributions (there will be a feature to exclude certain core
contributors from getting thanked all the time), and then I invite you
to go read the changes below, and hopefully you find any use for this
tool, as I have.

Be sure to check out the project [README] if you haven't already!

[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0-beta.4/
[annotated git tag]: https://git-scm.com/book/en/v2/Git-Basics-Tagging
[readme]: https://github.com/rustic-games/jilu/blob/master/README.md#%E8%AE%B0%E5%BD%95


### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Jean Mertz (<jean@mertz.fm>)

### Changes

#### Features

- **show commit type of unreleased changes** ([`10a3e99`])

  The goal is to keep the list of unreleased changes more succinct, but
  still provide enough information to see the most important upcoming
  changes at a glance.

  With this change, the commit type is added to the single-line
  description of each change, giving better insight into how relevant each
  change is.

  When breaking changes are added to the change log, that too should
  somehow be conveyed in the upcoming changes list, so people can
  anticipate these breaking changes by reading the change log.

- **configurable change log template and config** ([`d462839`])

  It's now possible to alter the way your change log looks by combining a
  set of configuration settings and an optional custom template system.

  The eventual goal is to support both use cases of making simple changes
  to the default template to suit your needs by using configuration
  settings, or to style your change log from scratch using your own
  template.

  Template support is fully supported with this change, and the first set
  of configuration settings are also added. More will follow in the
  future.

- **initial working version** ([`c62baf6`])

  This is a first "working" version of the `jilu` application.

  There are still many things to do, and there are still some hard-coded
  debug variables in the source code, but it's good to get it out there,
  and iterate on it from here on out.

  The three next big steps are:

  1. Improve testing setup.
  2. Expose configurables.
  3. Expand documentation.

  But it's out there, yay! 🎉 💃🏽

#### Bug Fixes

- **chronologically order release changes** ([`fa7f5a5`])

  Changes within a change set weren't ordered from newest to oldest, as
  was supposed to happen, this is now fixed.

  As an added bonus, this changes the commits returned by `git::commits`
  from a linked-list to a regular vector of chronologically ordered
  commits, improving performance and reducing complexity.

<!-- [releases] -->

[unreleased]: https://github.com/rustic-games/jilu/compare/v0.8.0...HEAD
[0.8.0]: https://github.com/rustic-games/jilu/releases/tag/v0.8.0
[0.7.0]: https://github.com/rustic-games/jilu/releases/tag/v0.7.0
[0.6.0]: https://github.com/rustic-games/jilu/releases/tag/v0.6.0
[0.5.0]: https://github.com/rustic-games/jilu/releases/tag/v0.5.0
[0.4.0]: https://github.com/rustic-games/jilu/releases/tag/v0.4.0
[0.3.0]: https://github.com/rustic-games/jilu/releases/tag/v0.3.0
[0.2.0]: https://github.com/rustic-games/jilu/releases/tag/v0.2.0
[0.1.1]: https://github.com/rustic-games/jilu/releases/tag/v0.1.1
[0.1.0]: https://github.com/rustic-games/jilu/releases/tag/v0.1.0

<!-- [commits] -->

[`83e7793`]: https://github.com/rustic-games/jilu/commit/83e7793ab13dcf9739f0d76684ef0e9c08b98ee4
[`95f91a0`]: https://github.com/rustic-games/jilu/commit/95f91a0d63864a40e7a424a0940461a8c0b90da7
[`a33ac8e`]: https://github.com/rustic-games/jilu/commit/a33ac8e9e846dea1904e62c268dd013a38f6d5a4
[`56ce427`]: https://github.com/rustic-games/jilu/commit/56ce427cf82b4a0aba51a197ba49aca894217426
[`1d15fe8`]: https://github.com/rustic-games/jilu/commit/1d15fe8e114561d88c46dfc528b95f6f39ab428c
[`83192b5`]: https://github.com/rustic-games/jilu/commit/83192b516d9d1eb0f0a0685b07bebc27468f0995
[`8c71b99`]: https://github.com/rustic-games/jilu/commit/8c71b996617925d0da7fd9dec1b7f26950dac8be
[`8b86c59`]: https://github.com/rustic-games/jilu/commit/8b86c593c583a1737fe7850e390acff154221f81
[`6cc1f0f`]: https://github.com/rustic-games/jilu/commit/6cc1f0fb49d81d3f78fe2500c91448800eab73d2
[`11ae252`]: https://github.com/rustic-games/jilu/commit/11ae252eabdd6b853b912ecad21feaac06639ebc
[`b598d90`]: https://github.com/rustic-games/jilu/commit/b598d906bff3805b77a3a4aea0ee4b6cb331c62b
[`a318e04`]: https://github.com/rustic-games/jilu/commit/a318e04133de8526f21553ff9c507f1d1bfa81bd
[`99695ca`]: https://github.com/rustic-games/jilu/commit/99695ca60eb4b99000b66933876cf8f5c78a3f90
[`65bab03`]: https://github.com/rustic-games/jilu/commit/65bab038934c5051572fa2a963caa53432b478f3
[`a4bbbdc`]: https://github.com/rustic-games/jilu/commit/a4bbbdc3a89923b815390f757b55c025e3f68d8d
[`f2c9f38`]: https://github.com/rustic-games/jilu/commit/f2c9f386be0e98c908810bcfab5d08101760b467
[`b101f8c`]: https://github.com/rustic-games/jilu/commit/b101f8caa52890b8776a13f5381696e7be2912be
[`e7e88b7`]: https://github.com/rustic-games/jilu/commit/e7e88b708d3c6e55fbd3b528cf2b22a431c6f47f
[`ceabf81`]: https://github.com/rustic-games/jilu/commit/ceabf81b2ebe24c9fea4f5d0ff95cf83e823c535
[`6b79f87`]: https://github.com/rustic-games/jilu/commit/6b79f8725ed99db07c5199428d8b70cf35bc9eb2
[`7ffd865`]: https://github.com/rustic-games/jilu/commit/7ffd8652fc633ba847f12c341cd4b8ddadea8660
[`3948e9c`]: https://github.com/rustic-games/jilu/commit/3948e9c096c9467f53315ab08bdfc86355d9b90b
[`adbabcf`]: https://github.com/rustic-games/jilu/commit/adbabcf6c75aedc58e10cdc60a1a9c80bedf4b1a
[`9ca8143`]: https://github.com/rustic-games/jilu/commit/9ca814329bf79e709fa10be5e76c944af0c02703
[`2e383d1`]: https://github.com/rustic-games/jilu/commit/2e383d181fe2a6634ed2bbb5292d6ec7d278533c
[`61ceeed`]: https://github.com/rustic-games/jilu/commit/61ceeed9d0fe6334a06e0c3334391ee339af8614
[`10a3e99`]: https://github.com/rustic-games/jilu/commit/10a3e9986f72281cdab675e3a94d3d80d62a10e3
[`fa7f5a5`]: https://github.com/rustic-games/jilu/commit/fa7f5a5853b579f179b30f70132bef6f151ed5a1
[`d462839`]: https://github.com/rustic-games/jilu/commit/d4628395305f87908d0ffcce13a657de4f88135c
[`c62baf6`]: https://github.com/rustic-games/jilu/commit/c62baf6627a3e0bb6d9c99ba93b9021caf083d6e

<!-- [pull requests] -->


<!--
Config(
  github: ( repo: "rustic-games/jilu" ),
  accept_types: ["feat", "fix", "perf"],
)
-->
