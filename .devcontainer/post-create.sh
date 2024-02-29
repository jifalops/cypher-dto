# Ensure devcontainer user owns the project directory
sudo chown -R developer:developer $PROJECT_ROOT

# Remember history on the local machine
ln -s $PROJECT_ROOT/.devcontainer/.bash_history ~/.bash_history

# Install dotfiles
gh repo clone dotfiles ~/dotfiles && ~/dotfiles/install.sh
