release VERSION:
    #!/usr/bin/env sh
    set -e

    export IGNORE_CONTRIBUTORS="jean@mertz.fm,git@jeanmertz.com"

    # Checks
    if ! git diff-index --quiet HEAD --; then
        echo >&2 "Dirty workspace. Commit or stash changes first."
        exit 1
    fi

    last_version=$(git describe --abbrev=0)
    if [ "$last_version" == "v{{VERSION}}" ]; then
        echo >&2 "Already on v{{VERSION}}. Nothing to do."
        exit 0
    fi

    # Update version references.
    cargo set-version {{VERSION}}
    sed -i '' -e "s/${last_version}/v{{VERSION}}/g" README.md
    git add .

    # Update change log, commit, and tag.
    jilu --release {{VERSION}} --edit --write --commit
    git push --tags

    # Create GitHub release, with assets.
    goreleaser release --clean

    # Set GitHub release subject/body, and mark as published.
    #
    # This is a "pro" feature of `goreleaser`. I'm willing to pay for good
    # software, but this is not worth $15/month, so here are four lines of code
    # that do the same thing.
    tag_subject=$(git for-each-ref refs/tags/v{{VERSION}} --format='%(subject)')
    tag_body=$(git for-each-ref refs/tags/v{{VERSION}} --format='%(body)')
    release_id=$(gh api \
        -H "Accept: application/vnd.github+json" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        -q "map(select(.draft == true))[0].id" \
        --paginate \
        "/repos/rustic-games/jilu/releases")

    gh api \
        --method PATCH \
        -H "Accept: application/vnd.github+json" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        /repos/rustic-games/jilu/releases/${release_id} \
        -f "name=${tag_subject}" -f "body=${tag_body}" -F "draft=false" --silent
