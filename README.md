<div align="center">

  <h1>记录</h1>
  <sup>jìlù, <em>to memorize</em></sup>
  <br />
  <br />

  <p>
    <strong><code>jilu</code> generates a change log based on the state of your Git repository.</strong>
  </p>

  <br />

  <p><sub>
    <em>convert <a href="https://www.conventionalcommits.org">conventional commits</a> into a <strong>human readable</strong> change log</em>
    <br />––––––––––<br />
    <em>use Git tags to <strong>annotate your releases</strong> with release titles and richly formatted release notes</em>
    <br />–––––<br />
    <em><strong>customize your change log template</strong> to best serve your community</em>
    <br />––––––––––<br />
    <em>integrate the <code>jilu</code> binary into your CI workflow for <strong>automated updates</strong></em>
  </sub></p>

  <br />
  <br />

</div>

The way you structure and document your projects is personal, and transforms
over time into what works best for you and your community. **Jilu** tries to
deliver a nice and simple way to expose your project changes to the outside
world. It's flexible, but it can never replace all possible workflows, and it
doesn't pretend to.

Using **Jilu** requires using [conventional commits], and while some like the
rigor of structured commits, others don't, both have valid reasons. If you have
an existing workflow that works for you, or an existing project without
conventional commits, there won't be much to gain for you here.

What matters most is the success of your project, with or without the help of
**Jilu**.

### Quick Start

1. Check out the [change log of this project].

2. Install `jilu`:

   - Using [release binaries]:

     ```shell
     curl -L https://github.com/rustic-games/jilu/releases/download/v0.3.0/jilu_0.3.0_$(uname)_x86_64.tar.gz | tar -xvf - jilu
     ```

   - ~~Using [Homebrew]~~ _(soon)_:

     ```shell
     brew install jilu
     ```

   - Using [Cargo]:

     ```shell
     cargo install jilu --git https://github.com/rustic-games/jilu
     ```

3. Visit any local repository:

   ```shell
   cd /path/to/repository
   ```

4. Print change log:

   ```shell
   jilu
   ```

5. ~~Get more details~~ _(soon)_:

   ```shell
   jilu --help
   ```

6. Integrate into your CI workflow.

[change log of this project]: ./CHANGELOG.md
[cargo]: https://www.rust-lang.org/tools/install
[homebrew]: https://brew.sh
[release binaries]: https://github.com/rustic-games/jilu/releases

### About

As a fan of [Conventional Commits], and an avid reader of open-source change
logs, It always saddens me when a change log is missing, or is lacking important
contextual details. Things like release dates, unreleased changes, or breaking
changes are an important part of the public documentation of a project. On top
of that, as an open-source contributor, I can't ignore the warm fuzzy feeling I
get when people thank me for my contributions, but keeping track of all those
contributions cuts into the time you have available for a project _(still,
automated thank-you's aren't as personal as an individual one, remember that)_.

Conventional commits and auto-generated change logs are quite popular in the
JavaScript community, and there are [tons][] [of][] [tools][] to help you adhere
to the standards, but all the ones I tried came with downsides, the biggest ones
being the lack of a single binary, lack of easy configuration, riddled with
Emoji _(I don't mind an Emoji or two, mind you ✌️)_, or trying to be a complete
release management tool, which inevitably means heavy focus on the JavaScript
ecosystem _(and not adhering to the Unix philosophy of small single-purpose
tools)_.

**Jilu** is the tool I envisioned I would like to use to generate my project
change logs, and I've open-sourced it so that it may help you in your
open-source endeavors as well. Have fun using it, and feel free to [propose new
features], or [provide bug fixes]!

[tons]: https://github.com/conventional-changelog/commitlint
[of]: https://commitizen.github.io/cz-cli/
[tools]: https://github.com/conventional-changelog/standard-version

[propose new features]: https://github.com/rustic-games/jilu/issues/new?title=[feature%20request]
[provide bug fixes]: https://github.com/rustic-games/jilu/issues/new

### Usage

Pipe the output of `jilu` to your change log file:

```
jilu > CHANGELOG.md
```

Note, if you've added [custom configurations] to your change log, this won't
work, as Unix will empty the `CHANGELOG.md` file first, before `jilu` can read
its contents, meaning it won't be able to read any existing configuration.

To work around this, use a tool like [`sponge`] to soak up the output of `jilu`
before redirecting it to the file:

```
jilu | sponge CHANGELOG.md
```

[custom configurations]: #configurable
[`sponge`]: https://linux.die.net/man/1/sponge

### Design

Want to know what makes **Jilu** tick? Read on.

#### Structured

The library uses existing conventions and specifications to structure the change
log. Specifically, you are expected to use [SemVer] for tagging releases,
[Conventional Commits] for commit messages, and the change log itself adheres to
the [Keep a Changelog] conventions.

#### Automated

Release notes are generated by parsing the Git history. Clone any repository
locally, and run `jilu` to print the generated release notes.

Commits are parsed using the [Conventional Commits] format.

Git tags are used to determine which commits should be part of what release. The
messages of annotated tags are used to add hand-written release notes to the
release. Similar to conventional commits, the first line of the tag annotation
is used as the release title, the rest as the release notes.

Any commits _after_ the latest tagged release are added to the "unreleased"
section.

**_work in progress_** ~~If a tag annotation contains a line starting with
`YANKED:`, it will be marked as such in the change log, with anything following
that marker being used as the reason for yanking the release. Git tag
annotations [can be replaced after pushing them][] (while retaining the tag date
and author), with some command-line-fu.~~

~~You can optionally run `jilu --release MAJOR.MINOR.PATCH` if you want to
generate a change log with the unreleased commits grouped in a new release. When
doing this, your `$EDITOR` will open to provide release notes. This will also
create a Git tag with the same details (unless you supply `--no-tag`).~~

~~The release feature is added to make sure the updated change log is part of
that new release. Otherwise, opening the change log in the repository state at
the point of a release tag would still show the new changes in the _unreleased_
section.~~

[can be edited after pushing them]: https://stackoverflow.com/a/29019547/747032

#### Readable

In the spirit of the [Keep a Changelog] guidelines, `jilu` produces a
human-readable change log by including the following information:

- [x] markdown formatting for improved readability
- [x] custom change log introduction paragraph
- [x] table of contents
- [x] unreleased changes at the top
- [x] release versions, titles and dates
- [x] release changes grouped by type (features, fixes, etc.)
- [x] manually written release notes
- [x] short git refs linking specific commits
- [x] optional thank-you's to contributors
- [x] optional GitHub linking to release/tag/compare views

#### Forgiving

Commit messages that need to be excluded can be, based on a set of rules:

- [x] non-conventional commit messages are ignored
- [x] conventional commit types (such as `chore`) can be excluded
- [ ] **_work in progress_** ~~a list of blacklisted commits can be provided~~
- [ ] **_work in progress_** ~~a root commit can be provided to ignore older
      commits~~

#### Configurable

**Jilu** has a powerful configuration system that stays out of your way when you
don't need it, but allows you to automate the construction of your change log
in a way that works best for your project or community.

You can:

- [x] set header names for grouped changes (features, fixes, etc.)
- [x] ignore specific commit types
- [x] fully customize the change log template

You can check out the bottom of [this project's change log] for its
configuration, and [the default template][tpl] to see how the templating system
works.

Here's a quick example of what's possible (this snippet goes at the bottom of
your own `CHANGELOG.md` file):

```markdown
<!--
Config(
  github: ( repo: "rustic-games/jilu" ),
  accept_types: ["feat", "fix", "perf"],
  type_headers: {
    "feat": "Features",
    "fix": "Bug Fixes",
    "perf": "Performance Improvements"
  }
)

Template(
# My Change Log

## Upcoming Changes

{% for change in unreleased.changes %}
- {{ change.description }} ([`{{ change.commit.short_id }}`])
{%- endfor %}
)
-->
```

Putting the configuration _inside_ the change log file itself ensures that the
configuration can be read by **Jilu**, but won't show up in the markdown
rendered document and is easy to ignore in text format, since it will always be
at the end of the change log. It also means you don't need to add _another_
configuration file to your Git repository root.

The templating system uses the [Tera] library to provide Django-like syntax. If
no template is defined, the [default template][tpl] is used instead.

[this project's change log]: https://raw.githubusercontent.com/rustic-games/jilu/master/CHANGELOG.md
[tera]: https://tera.netlify.com/
[tpl]: https://raw.githubusercontent.com/rustic-games/jilu/master/template.md
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0-beta.4/
[semver]: https://semver.org/
[keep a changelog]: https://keepachangelog.com/en/1.0.0/
