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
    VERSION=$(git-cliff --bump --unreleased | grep -m1 "## \[" | sed -E 's/## \[(.*)\].*/\1/')
    echo "Preparing release v${VERSION}..."
    git switch -c release/v${VERSION}
    git-cliff --tag v${VERSION} --prepend CHANGELOG.md
    uv version "${VERSION}" --frozen
    sed -i 's/^version = "[0-9.]*"/version = "'"${VERSION}"'"/' Cargo.toml
    echo "Release preparation complete. Review changes, commit, and push the branch."

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
