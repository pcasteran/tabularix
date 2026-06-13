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

# Build the Python extension module
[group("dev")]
build:
    maturin develop

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
    uvx zensical build

# Serve the documentation site locally
[group("docs")]
docs-serve:
    uvx zensical serve
