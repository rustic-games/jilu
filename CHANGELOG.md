# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning]. The file is auto-generated using [Conventional Commits].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0-beta.4/

## Overview

- [unreleased](#unreleased)
- [`0.1.0`](#010) – _2019.08.12_

## _[Unreleased]_

- fix: prevent subtraction with overflow ([`2e383d1`])
- fix: correctly check for no new changes ([`61ceeed`])

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

[unreleased]: https://github.com/rustic-games/jilu/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/rustic-games/jilu/releases/tag/v0.1.0

<!-- [commits] -->

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
