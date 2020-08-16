# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning]. The file is auto-generated using [Conventional Commits].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0-beta.4/

## Overview

- [unreleased](#unreleased)
- [`0.4.0`](#040) – _2020.08.16_
- [`0.3.0`](#030) – _2020.03.14_
- [`0.2.0`](#020) – _2019.09.17_
- [`0.1.1`](#011) – _2019.08.12_
- [`0.1.0`](#010) – _2019.08.12_

## _[Unreleased]_

_nothing new to show for… yet!_

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

[unreleased]: https://github.com/rustic-games/jilu/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/rustic-games/jilu/releases/tag/v0.4.0
[0.3.0]: https://github.com/rustic-games/jilu/releases/tag/v0.3.0
[0.2.0]: https://github.com/rustic-games/jilu/releases/tag/v0.2.0
[0.1.1]: https://github.com/rustic-games/jilu/releases/tag/v0.1.1
[0.1.0]: https://github.com/rustic-games/jilu/releases/tag/v0.1.0

<!-- [commits] -->

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

<!--
Config(
  github: ( repo: "rustic-games/jilu" ),
  accept_types: ["feat", "fix", "perf"],
)
-->
