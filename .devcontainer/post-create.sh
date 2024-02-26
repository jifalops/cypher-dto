# Ensure devcontainer user owns the project directory
sudo chown -R vscode:vscode /cypher-dto

# Remember history on the local machine
ln -s /cypher-dto/.devcontainer/.bash_history ~/.bash_history

# Install dotfiles
gh repo clone dotfiles ~/dotfiles && ~/dotfiles/install.sh

# rustup and cargo bash completion.
sudo apt-get update -qq && sudo apt-get install -y -qq --no-install-recommends bash-completion \
  && mkdir -p ~/.local/share/bash-completion/completions \
  && rustup completions bash > ~/.local/share/bash-completion/completions/rustup \
  && rustup completions bash cargo > ~/.local/share/bash-completion/completions/cargo
