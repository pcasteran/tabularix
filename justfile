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

#
# Development recipes
#

# Build the Docker image of the devcontainer (for debug purposes)
[group("dev")]
devcontainer-build:
    docker build -f ./.devcontainer/Dockerfile .

# Execute the pre-commit hooks using prek
[group("dev")]
prek:
    prek run --all-files

# Update the pre-commit hooks
[group("dev")]
prek-hooks-update:
    prek autoupdate

# Upgrade the tools used in the project
[group("dev")]
upgrade-toolchain:
    mise upgrade
    mise lock

# Upgrade all project dependencies
[group("dev")]
upgrade-all: upgrade-toolchain prek-hooks-update

# Pin the GitHub Actions hash
[group("dev")]
gha_pin:
    pinact run

# Update the GitHub Actions
[group("dev")]
gha_update:
    pinact run -update
