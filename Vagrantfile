# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "ubuntu/trusty64"

  # Create a private network, which allows host-only access to the machine
  # using a specific IP.
  # config.vm.network "private_network", ip: "192.168.33.10"

  # Create a public network, which generally matched to bridged network.
  # Bridged networks make the machine appear as another physical device on
  # your network.
  # config.vm.network "public_network"

  config.vm.synced_folder ".", "/vagrant"

  # Install rust
  config.vm.provision "shell", inline: <<-SHELL
   curl -sSf https://static.rust-lang.org/rustup.sh > rustup.sh
   chmod +x rustup.sh
   ./rustup.sh -y
  SHELL

  # Install procs dev header
  config.vm.provision "shell", inline: <<-SHELL
   sudo apt-get update
   sudo apt-get install libprocps3-dev
  SHELL

end
