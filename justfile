[private]
default: help

# Load the environment variables defined in the `.env` file.

set dotenv-load

#
# Miscellaneous recipes
#

# Print the available recipes
[group("misc")]
help:
    @just --justfile {{ justfile() }} --list

# Upgrade all project dependencies
[group("misc")]
upgrade-all: upgrade-toolchain prek-hooks-update gha_update

# Prepare a new release (calculate version, create branch, update changelog and manifests)
[group("misc")]
prepare-release:
    #!/usr/bin/env bash
    set -euo pipefail

    # 1. Compute the next version number using git-cliff bump calculation
    VERSION=$(git-cliff --bump --unreleased | grep -m1 "## \[" | sed -E 's/## \[(.*)\].*/\1/')
    echo "Preparing release v${VERSION}..."

    # 2. Switch to a dedicated release branch, creating it if it doesn't exist
    TARGET_BRANCH="chore/release-v${VERSION}"
    if [ "$(git branch --show-current)" != "${TARGET_BRANCH}" ]; then
        git switch -c "${TARGET_BRANCH}" || git switch "${TARGET_BRANCH}"
    fi

    # 3. Generate and prepend the new changelog section to CHANGELOG.md
    git-cliff --tag v${VERSION} --unreleased --prepend CHANGELOG.md

    # 4. Bump the Python package version statically in pyproject.toml
    uv version "${VERSION}" --frozen

    # 5. Bump the Rust crate version in Cargo.toml
    sed -i 's/^version = "[0-9.]*"/version = "'"${VERSION}"'"/' Cargo.toml

    # Run formatting and validation checks to ensure clean updates
    just prek

    # 6. Display complete status and post-merge publishing instructions
    echo "Release preparation complete. Review changes, commit, and push the branch."
    echo ""
    echo "Once the PR is merged, run the following commands to tag and trigger the release pipeline:"
    echo "  git checkout main"
    echo "  git pull"
    echo "  git tag v${VERSION}"
    echo "  git push origin v${VERSION}"

#
# Development recipes
#

# Build the Docker image of the devcontainer (for debug purposes)
[group("dev")]
devcontainer-build:
    docker build -f ./.devcontainer/Dockerfile .

# Upgrade the tools used in the project
[group("dev")]
upgrade-toolchain:
    mise upgrade
    mise lock

# Build the Python extension module using Maturin and uv
[group("dev")]
build:
    uv run maturin develop

#
# Static analysis recipes
#

# Execute the pre-commit hooks using prek
[group("static analysis")]
prek:
    prek run --all-files

# Update the pre-commit hooks
[group("static analysis")]
prek-hooks-update:
    prek auto-update

#
# CI/CD recipes
#

# Pin the GitHub Actions hash
[group("ci/cd")]
gha_pin:
    pinact run

# Update the GitHub Actions
[group("ci/cd")]
gha_update:
    pinact run -update

#
# Tests recipes
#

# Run the Rust internal unit tests
[group("test")]
unit-test:
    cargo test

# Run the Robot Framework acceptance tests
[group("test")]
acceptance-test: build
    uv run robot -d results tests/

#
# Documentation recipes
#

# Build the documentation site
[group("docs")]
docs-build:
    uvx --with mkdocstrings-python zensical build --clean

# Serve the documentation site locally
[group("docs")]
docs-serve:
    uvx --with mkdocstrings-python zensical serve
