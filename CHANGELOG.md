# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning]. The file is auto-generated using [Conventional Commits].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0/

## Overview

- [unreleased](#unreleased)
- [`0.13.1`](#0.13.1) – _2025.06.11_
- [`0.13.0`](#0.13.0) – _2025.06.11_
- [`0.12.0`](#0.12.0) – _2025.06.11_
- [`0.11.0`](#0.11.0) – _2025.06.11_
- [`0.10.0`](#0.10.0) – _2025.06.10_
- [`0.9.0`](#0.9.0) – _2025.06.10_
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

<a id="0.13.1" />

## [0.13.1]

_2025.06.11_
### Changes

#### Bug Fixes

- **remove contributor empty lines in default template** ([`fcbb1d3`])

<a id="0.13.0" />

## [0.13.0] – _The Simpsons Release_

_2025.06.11_

A release to improve contribution tracking, with special guests.

### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Bart Simpson (<bart@simpsons.com>)
- Homer Simpson (<homer@simpsons.com>)
- Lisa Simpson (<lisa@simpsons.com>)
- Marge Simpson (<marge@simpsons.com>)

### Changes

#### Features

- **Support contributor footers in changelog generation** ([`11bc91d`])

  This change enhances contributor tracking by expanding beyond just
  commit authors to include contributors listed in commit footers. The
  system now recognizes common footer tokens like `Co-authored-by`,
  `Signed-off-by`, `Reviewed-by`, and others to create a comprehensive
  list of contributors for each change.

  The list of contributor footers can be configured using the
  `contributor_footers` configuration option.

  The list defaults to:

  - co-authored-by
  - signed-off-by
  - reported-by
  - tested-by
  - reviewed-by
  - suggested-by
  - acked-by

  The author and committer of a change are always included in the list of
  contributors, contributors are sorted alphabetically, and duplicate
  entries are removed.

<a id="0.12.0" />

## [0.12.0] – _A promise made is a promise kept._

_2025.06.11_

It took the better part of five years to keep this promise, but here we
are.

### Changes

#### Features

- **Add support for ignoring specific commits** ([`e9b2784`])

  Users can now configure the `ignore_commits` field in their
  configuration to provide a list of commit hashes that will be filtered
  out during change log generation.

- **Add root commit filtering for changelog generation** ([`1d01661`])

  This change introduces the ability to specify a `root_commit` in the
  configuration to limit changelog generation to commits newer than the
  specified commit.

<a id="0.11.0" />

## [0.11.0]

_2025.06.11_

This release allows release notes without subjects. If a tag message
contains multiple lines without a newline between the first and second
line, the entire message will be used as part of the release note,
instead of treating the first line as the release title.

### Changes

#### Bug Fixes

- **Improve git tag message parsing for subject extraction** ([`7f7854f`])

  The previous implementation incorrectly parsed multi-line git tag
  messages, failing to distinguish between subject and body when there
  wasn't a proper blank line separator. This caused some tags to be
  treated as having a subject when they didn't.

  The updated logic now properly handles the Git convention where a tag
  message must have an empty line between the subject and body. If there's
  no empty second line followed by additional content, the entire message
  is treated as body-only, this allows for untitled releases that still
  have release notes attached to the tag.

<a id="0.10.0" />

## [0.10.0]

_2025.06.10_
### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- Jean Mertz <git@jeanmertz.com>

$ ------------------------ >8 ------------------------
$ Do not modify or remove the line above.
$ Everything below it will be ignored.
$
$ On branch main
$ Your branch is ahead of 'origin/main' by 1 commit.
$   (use "git push" to publish your local commits)
$
$ Changes to be committed:
$	modified:   src/changelog.rs
$	modified:   src/main.rs
$
$ Changes not staged for commit:
$	modified:   src/main.rs
$
$ Untracked files:
$	.github/templates/tag.md
$
diff --git a/src/changelog.rs b/src/changelog.rs
index 9911764..8ad1600 100644
--- a/src/changelog.rs
+++ b/src/changelog.rs
@@ -62,7 +62,7 @@ impl<'a> Changelog<'a> {
         &self.unreleased
     }

-    pub fn render(&self) -> Result<String, Error> {
+    pub fn render(&self, include_metadata: bool) -> Result<String, Error> {
         let context = tera::Context::from_serialize(self)?;
         let mut tera = tera::Tera::default();
         let template = self
@@ -80,8 +80,10 @@ impl<'a> Changelog<'a> {
         tera.register_filter("scopeheader", scope_header);

         let mut log = tera.render("template", &context)?;
-        if let Some(metadata) = &self.config.metadata {
-            log.push_str(&format!("\n{}\n", metadata));
+        if include_metadata {
+            if let Some(metadata) = &self.config.metadata {
+                log.push_str(&format!("\n\n{}\n", metadata));
+            }
         }

         Ok(log)
diff --git a/src/main.rs b/src/main.rs
index 3d009ce..8bad0d9 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -52,6 +52,9 @@ struct Opts {
     /// `stdout`.
     write: bool,

+    /// If set, the change log will be rendered without inline configuration.
+    strip_config: bool,
+
     /// Output the release notes either in `text` or `json` format. Defaults to
     /// `text`, unless `write` is set, in which case it defaults to `none`.
     output: Option<String>,
@@ -78,6 +81,7 @@ impl Opts {
         use lexopt::{Arg::*, ValueExt as _};

         let mut write = false;
+        let mut strip_config = false;
         let mut output = None;
         let mut output_file = None;
         let mut file = None;
@@ -115,6 +119,9 @@ impl Opts {
                 Short('e') | Long("edit") => {
                     edit_release_notes = true;
                 }
+                Long("strip-config") => {
+                    strip_config = true;
+                }
                 Short('h') | Long("help") => {
                     println!("Usage: jilu [-r|--release=VERSION] [-n|--notes=RELEASE_NOTES] [-e|--edit] [-w|--write] [CHANGELOG]");
                     std::process::exit(0);
@@ -143,6 +150,7 @@ impl Opts {
         Ok(Self {
             file,
             write,
+            strip_config,
             output,
             output_file,
             release,
@@ -175,11 +183,11 @@ fn run(opts: Opts) -> Result (<String, Error> {
     let log = Changelog::new(&config, &commits, tags)?;

     if opts.write {
-        std::fs::write(&opts.file, log.render()?)?;
+        std::fs::write(&opts.file, log.render(!opts.strip_config)?)?;
     }

     match (opts.output.as_deref(), opts.jq.as_deref()) {
-        (Some("text"), _) => Ok(log.render()?),
+        (Some("text"), _) => Ok(log.render(!opts.strip_config)?),
         (Some("json"), None) => Ok(serde_json::to_string(&log)?),
         (Some("json"), Some(code)) =>)

### Changes

#### Bug Fixes

- **handle empty release notes** ([`f0274d6`])

- **write to stdout by default, as intended** ([`e08fb2c`])

- **resolve newline formatting in default template** ([`c3ae8ed`])

#### Features

- **`--strip-config` excludes metadata from rendered changelog** ([`b0dcc8a`])

  This change introduces a new command line option that allows users to
  generate changelog output without the inline configuration metadata.
  When the `--strip-config` flag is provided, the rendered changelog will
  omit any metadata section that would normally be appended.

  This is useful when you want to render a template to stdout without
  overwriting the file itself that contains the metadata. You can use this
  to have dedicated "template" files that Jilu can use to render to
  stdout. For example; if you have a `tag.md` file that contains only
  metadata with a custom `Template` section, you can use it to render the
  contents of a tag, pipe the output to a file, and use that file as the
  input for the tag message.

<a id="0.9.0" />

## [0.9.0] – _Back to our roots!_

_2025.06.10_

In the last few days, I dug Jilu in a hole that kept getting deeper. It
started with the question “how can I fit Jilu in the release flow I
want”, and ended with “now, how do I make Jilu create signed annotated
tags?”

Jilu is *not* a release management tool. It is a change log generator.
Jilu needs access to Git, yes, but it should *never* need write access
to the repository. All it needs is a list of commits, and a list of
existing tags, nothing else. In fact, we could just have Jilu itself
never interact with Git and feed it the commits and tags data through
external means, but that would tip the scales in the other direction:
making Jilu too minimalist for its own good.

Instead, the right balance is this:

- Jilu is a change log generator
- It needs read access to a Git repository
- It will never change any Git objects
- It will only write to the change log file, if requested
- It can be used as a cog in a larger release workflow

This last item is what drove me down this rabbit hole, but here we are,
back at the surface, this release officially seals the hole, and puts us
on a much saner path.

So how can we still achieve that last item, without having write access
to Git? Well, we _push_ data instead of _pull_, meaning, when you run
`jilu`, you can now ask it for structured (JSON) output with all the
relevant change log data you could need, to feed into your release
workflow.

This is done through the new `--output` flag, which can be either
`text`, `json`, or `none`. If you call `jilu --write`, then `--output`
is set to `none`, If you call it as `jilu` then output is set to `text`,
unless you explicitly set it to `none`. This is how things worked
implicitly until this release. Now, there’s a new `json` option, which
gives you structured data of the generated change log:

```sh
jilu --output=json
```

```json
{
  "config": {
    "github": {
      "repo": "rustic-games/jilu"
    },
    "accept_types": [...],
    "type_headers": {
      "refactor": "Refactoring",
      ...
    }
  },
  "unreleased": {
    "changes": [],
  },
  "releases": [
    {
      "version": "0.8.0",
      "subject": "Commit or don't, there is no such thing as a free lunch.",
      "notes": "Swap `--tag` for `--commit`, bringing us closer to a proper release\nworkflow.",
      "date": "2025-06-09T14:47:02Z",
      "changeset": {
        "changes": [
          {
            "type": "feat",
            "scope": "changelog",
            "description": "Add commit functionality to release workflow",
            "body": "...",
            "commit": {
              "short_id": "83e7793",
              "id": "83e7793ab13dcf9739f0d76684ef0e9c08b98ee4"
            }
          }
        ],
        "contributors": [
          {
            "name": "Jean Mertz",
            "email": "git@jeanmertz.com"
          }
        ]
      }
    },
    ...
  ]
}
```

As you can see, there’s a lot of data you can use in external tools. One
obvious use-case is creating a release commit and tag, which is what we
introduced in 0.8.0, and are now removing again.

You can use a tool like `jq` to query this data in a script you
maintain, or you can use the new `--jq` flag to have Jilu do the JSON
filtering, without needing to install `jq`. This is mostly a convenience
feature for those that don’t have Jq installed.

Here’s how you can create the relevant release commit and tag in a shell
script:

```sh
# Create a temporary file to store the change log JSON output. This is
# required, because using `--edit` requires stdout to be a TTY for our
# editor, so we need to write the JSON output to a file instead of
# stdout.
release=$(mktemp)

# 1. We group the unreleased changes in a new release 0.9.0
# 2. We edit the release notes in our $EDITOR
# 3. We write the changes to our CHANGELOG.md
# 4. We return the change log data as JSON to stdout
# 5. We filter the JSON to the latest release (0.9.0)
# 6. We write the JSON output to our temporary file
jilu --release=v0.9.0 \
     --edit \
     --write \
     --output=json \
     --jq='.releases[0]' \
     --output-file="$release"

# 1. We make sure to stage the change log changes
git add CHANGELOG.md

# 1. We use `jq -r` to produce a raw (unquoted) string for us
# 2. We create a new release commit message
# 3. We commit the staged changes
msg=$(echo "$release" | jq -r '"chore: Release v" + .version')
git commit --message "$msg"

# 1. We create a new release tag message
# 2. We create the tag
msg=$(echo "$release" | jq -r '[.subject, .notes] | join("\n\n")')
git tag --sign --message "$msg"
```

This is one example, but it shows how you can use the structured data
any way you want in your release pipeline, without Jilu becoming a
bloated complicated tool that does too many things poorly, instead of a
dedicated tool that does a few things extremely well.

### Changes

#### Features

- **Add JSON output with JQ filtering support** ([`e52d264`])

  Transform Jilu from a release management tool into a pure changelog
  generator by removing Git write capabilities and introducing structured
  JSON output. Users can now extract changelog data programmatically
  through the new `--output=json` flag and filter it using the `--jq` flag
  for integration into custom release workflows. A `--output-file` flag is
  available to allow using `--write` and `--output` together.

  The new JSON output includes complete changelog metadata including
  configuration, unreleased changes, and release history with all
  associated commit and contributor information. This enables external
  tools to consume structured changelog data without requiring Jilu to
  have write access to Git repositories.

  The `--output` flag supports three modes: `text` (default for display),
  `json` (structured data), and `none` (when using `--write`). The `--jq`
  flag provides built-in JSON filtering without requiring external `jq`
  installation, including a custom `raw` function for unquoted string
  output.

#### Bug Fixes

- **properly render release headings on Github** ([`391a952`])

<a id="0.8.0" />

## [0.8.0] – _Commit or don't, there is no such thing as a free lunch._

_2025.06.09_

Swap `--tag` for `--commit`, bringing us closer to a proper release
workflow.

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

[unreleased]: https://github.com/rustic-games/jilu/compare/v0.13.1...HEAD
[0.13.1]: https://github.com/rustic-games/jilu/releases/tag/v0.13.1
[0.13.0]: https://github.com/rustic-games/jilu/releases/tag/v0.13.0
[0.12.0]: https://github.com/rustic-games/jilu/releases/tag/v0.12.0
[0.11.0]: https://github.com/rustic-games/jilu/releases/tag/v0.11.0
[0.10.0]: https://github.com/rustic-games/jilu/releases/tag/v0.10.0
[0.9.0]: https://github.com/rustic-games/jilu/releases/tag/v0.9.0
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

[`fcbb1d3`]: https://github.com/rustic-games/jilu/commit/fcbb1d39f081ce962e47b2fd7a7093be387ca0d2
[`11bc91d`]: https://github.com/rustic-games/jilu/commit/11bc91dccf1880f38b2ff0c496611ad5ca684eb8
[`e9b2784`]: https://github.com/rustic-games/jilu/commit/e9b2784f5a7d769bebdc45aa11b995fc967352a5
[`1d01661`]: https://github.com/rustic-games/jilu/commit/1d01661320eab9fc538c104866aa098e7ae48dc7
[`7f7854f`]: https://github.com/rustic-games/jilu/commit/7f7854f88df5c5c96f1f8ecc40cfbc930cc066d0
[`f0274d6`]: https://github.com/rustic-games/jilu/commit/f0274d6d85f30edad1659bcb8fe2396d2e87f1e7
[`e08fb2c`]: https://github.com/rustic-games/jilu/commit/e08fb2c5f5183de3508832c63b627d29cce98cce
[`b0dcc8a`]: https://github.com/rustic-games/jilu/commit/b0dcc8a5e9eb4d6ea1a1c60156f34e0a6a61e6db
[`c3ae8ed`]: https://github.com/rustic-games/jilu/commit/c3ae8ed436504756aee87a82be06ec4b8ade2627
[`e52d264`]: https://github.com/rustic-games/jilu/commit/e52d26449f430dbf8a066d9d005cee8a273f1873
[`391a952`]: https://github.com/rustic-games/jilu/commit/391a9525a6b824e81073be36d6570fb456eba51e
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
