FROM ubuntu:24.04

# 安装基础工具
RUN apt-get update && apt-get install -y \
  git \
  curl \
  wget \
  unzip \
  openssh-server \
  build-essential \
  zsh \
  && rm -rf /var/lib/apt/lists/*

# 安装 Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# 安装 code-server 和插件
RUN curl -fsSL https://code-server.dev/install.sh | sh && \
    code-server --install-extension cnbcool.cnb-welcome && \
    code-server --install-extension redhat.vscode-yaml && \
    code-server --install-extension waderyan.gitblame && \
    code-server --install-extension mhutchie.git-graph && \
    code-server --install-extension donjayamanne.githistory && \
    code-server --install-extension cloudstudio.live-server && \
    code-server --install-extension tencent-cloud.coding-copilot && \
    code-server --install-extension rust-lang.rust-analyzer

# 安装 clippy
RUN rustup component add clippy

# 安装 Oh My Zsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# 安装 Oh My Zsh 插件
RUN git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting && \
    git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions

# 配置 Zsh
RUN sed -i 's/plugins=(git)/plugins=(git zsh-syntax-highlighting zsh-autosuggestions)/' ~/.zshrc && \
    chsh -s /bin/zsh

# 设置环境变量
ENV LANG C.UTF-8
ENV LANGUAGE C.UTF-8
