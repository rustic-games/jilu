run *ARGS:
    #!/usr/bin/env sh
    set -e

    export IGNORE_CONTRIBUTORS="jean@mertz.fm,git@jeanmertz.com"
    cargo run -- {{ARGS}}

release VERSION: _check-git-index _check-goreleaser (_install "cargo-edit@^0.13" "jaq@^2.2")
    #!/usr/bin/env sh
    set -e
    version="{{VERSION}}"

    # Get the last annotated git tag.
    last_version="$(git describe --abbrev=0)"
    if [ "$last_version" == "v$version" ]; then
        echo >&2 "Already on v$version. Nothing to do."
        exit 0
    fi

    # Update version references.
    cargo set-version "$version"
    sed -i '' -e "s/${last_version}/v${version}/g" README.md

    # Create a temporary file to store the change log JSON output
    release="$(mktemp)"

    # 1. Group the unreleased changes in a new release
    # 2. Edit the release notes in our $EDITOR
    # 3. Write the changes to our CHANGELOG.md
    # 4. Output the change log data as JSON
    # 5. Json query the change log to the latest release
    # 6. Output the change log data to the temporary file
    just run \
        --release="$version" \
        --edit \
        --write \
        --output=json \
        --jq='.releases[0]' \
        --output-file="$release"

    # Make sure to stage all new changes.
    git add .

    # Commit a new release commit
    git commit --signoff --message "chore: Release v$version"

    # 1. Create a new release tag message
    # 2. Create the tag
    # 3. Push the latest commit and tag
    msg="$(jaq -r '[.subject, .notes] | join("\n\n") | trim' $release)"
    git tag --sign --message "$msg" "v$version"
    git push --tags
    git push

    # 1. Create a new realese note subject and body
    # 1. Set environment variables to use in `.goreleaser.yml` templates
    # 2. Run `goreleaser` to create the release on GitHub
    msg="$(just run --strip-config .github/templates/tag.md)"
    export GORELEASER_RELEASE_SUBJECT="$(echo "$msg" | head -n1)"
    export GORELEASER_RELEASE_NOTES="$(echo "$msg" | tail -n+3)"
    goreleaser release --clean

# Make sure there are no uncommitted changes.
_check-git-index:
    #!/usr/bin/env sh
    if ! git diff-index --quiet HEAD --; then
        echo >&2 "Dirty workspace. Commit or stash changes first."
        exit 1
    fi

# Make sure goreleaser is installed.
_check-goreleaser:
    #!/usr/bin/env sh
    if ! command -v goreleaser >/dev/null 2>&1; then
        echo >&2 "goreleaser is not installed. Visit https://goreleaser.com/ to install it."
        exit 1
    fi

@_install +CRATES: _install-binstall
    cargo binstall --locked --quiet --disable-telemetry --no-confirm --only-signed {{CRATES}}

@_install-binstall:
    cargo install --locked --quiet --version ^1.12 cargo-binstall
