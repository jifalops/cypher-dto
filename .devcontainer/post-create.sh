# Ensure devcontainer user owns the project directory
sudo chown -R vscode:vscode /cypher-dto

# Remember history on the local machine
ln -s /cypher-dto/.devcontainer/.bash_history ~/.bash_history

# Install dotfiles
gh repo clone dotfiles ~/dotfiles && ~/dotfiles/install.sh
