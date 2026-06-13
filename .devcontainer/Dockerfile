FROM mcr.microsoft.com/devcontainers/python:3.14

ARG USERNAME=vscode

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Install mise.
RUN curl https://mise.run | MISE_INSTALL_PATH=/usr/local/bin/mise sh

# Switch to devcontainer user.
USER ${USERNAME}

ARG USER_HOME="/home/${USERNAME}"
ARG USER_CACHE_DIR="${USER_HOME}/.cache"

# Install the Rust toolchain.
ENV CARGO_HOME="${USER_HOME}/.cargo"
ENV RUSTUP_HOME="${USER_HOME}/.rustup"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal -c rustfmt,clippy
ENV PATH="${CARGO_HOME}/bin:${PATH}"

# Install the tools from mise.toml.
COPY --chown=${USERNAME}:${USERNAME} mise.toml /home/${USERNAME}/.config/mise/config.toml
COPY --chown=${USERNAME}:${USERNAME} mise.lock /home/${USERNAME}/.config/mise/mise.lock

RUN mise install --locked

# Install Antigravity CLI.
RUN curl -fsSL https://antigravity.google/cli/install.sh | bash

# Install custom git commands.
COPY .devcontainer/git-custom-cmd/git-* /usr/local/bin/

# Create a volume for some cache folder(s).
ARG PREK_HOME="${USER_CACHE_DIR}/prek"
RUN mkdir -p "${PREK_HOME}"
VOLUME ["${PREK_HOME}"]

ARG GEMINI_CLI_CACHE="${USER_HOME}/.gemini"
RUN mkdir -p "${GEMINI_CLI_CACHE}"
VOLUME ["${GEMINI_CLI_CACHE}"]

# Misc.
# hadolint ignore=SC2016
RUN echo 'alias ll="ls -al"' >> ~/.bashrc && \
    echo 'eval "$(mise activate bash)"' >> ~/.bashrc
