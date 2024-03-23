# Ensure devcontainer user owns the project directory
GIT_ROOT="$(sudo git rev-parse --show-toplevel)"
sudo chown -R developer:developer $GIT_ROOT

# Remember history on the local machine
ln -s $GIT_ROOT/.devcontainer/.bash_history ~/.bash_history

# Install dotfiles
gh repo clone dotfiles ~/dotfiles && ~/dotfiles/install.sh
