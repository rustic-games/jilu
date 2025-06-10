run *ARGS:
    #!/usr/bin/env sh
    set -e

    export IGNORE_CONTRIBUTORS="jean@mertz.fm,git@jeanmertz.com"
    cargo run -- {{ARGS}}

release VERSION:
    #!/usr/bin/env sh
    set -e
    version="{{VERSION}}"

    # Make sure there are no uncommitted changes.
    if ! git diff-index --quiet HEAD --; then
        echo >&2 "Dirty workspace. Commit or stash changes first."
        exit 1
    fi

    # Get the last annotated git tag.
    last_version=$(git describe --abbrev=0)
    if [ "$last_version" == "v$version" ]; then
        echo >&2 "Already on v$version. Nothing to do."
        exit 0
    fi

    # Update version references.
    cargo set-version "$version"
    sed -i '' -e "s/${last_version}/v${version}/g" README.md

    # Create a temporary file to store the change log JSON output
    release=$(mktemp)

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
    msg=$(jq -r '[.subject, .notes] | join("\n\n")' "$release")
    changes=$(echo "$release" | jq -r '.changeset.changes | map("`" + .commit.short_id + "` " + .type + if .scope then "(" + .scope + ")" else "" end + ": " + .description) | join("\n")')

    git tag --sign --message "$msg\n\n$changes" "v$version"
    git push --tags
    git push

    # 1. Set environment variables to use in `.goreleaser.yml` templates
    # 2. Run `goreleaser` to create the release on GitHub
    export GORELEASER_RELEASE_SUBJECT=$(echo "$release" | jq -r '.subject')
    export GORELEASER_RELEASE_NOTES=$(echo "$release" | jq -r '.notes')
    goreleaser release --clean
