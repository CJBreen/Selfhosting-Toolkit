#!/usr/bin/bash

echo Installing Nextcloud...
sleep 1
echo Nextcloud will be installed by default in your home directory.
echo Would you like to choose a name for the folder? Leave blank for the default and press [ENTER]:
# get the user input
read location

# if they leave it blank
if [[ ${#location} == 0 ]]; then
  echo Installing to SelfService/Nextcloud
  rootdir="/home/cbreen/SelfService"
  sleep .5
  if [[ -d "$rootdir" ]]; then
    echo SelfService folder exists. Creating Nextcloud Folder.
    mkdir ~/SelfService/Nextcloud
    sleep .5
    echo Done.
  else
    mkdir ~/SelfService
    mkdir ~/SelfService/Nextcloud
    echo Created "SelfService/Nextcloud"
  fi

  # if they specify a location for the installation path
else
  echo Installing to $location/Nextcloud
  if [[ -d "$location" ]]; then
    mkdir ~/$location/Nextcloud
  else
    echo $location does not exist. Creating...
    sleep .5
  fi

fi

# Detect distro
# i am using the os-release file to check for the name of the
# distribution
if [[ -f /etc/os-release ]]; then
  distro=$(source /etc/os-release && echo $NAME)
else
  distro="*Unknown*"
  # exit with error
fi

echo "Detected distro: $distro"

# downloading tools
# we use the saved package manager to download docker and docker-compose
#
#installing docker using the recommended docker script
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# checking for the package manager on the distribution
# using the result from the previous function, we can determine
# the installed package manager.
#
# these will have stable versions of Docker
echo -n "Current package manager: "

# if the distro is either Ubuntu, Debian or Linux Mint
if [[ "$distro" == "Ubuntu" || "$distro" == "Debian" || "$distro" == "Linux Mint" ]]; then
  echo "installing from apt..."
  sudo apt install docker-compose-plugin
  # if the distribution is either Fedora, or Red Hat
elif [[ "$distro" == "Fedora" || "$distro" == "Red Hat"* ]]; then
  echo "installing from dnf..."
  sudo fnd install docker-compose
  # if the distribution is CentOS
elif [[ "$distro" == "CentOS" ]]; then
  echo "installing from yum..."
  sudo yum install docker-compose-plugin
  # if the distribution is either Arch, or the 2 popular forks (Manjaro and Garuda Linux, respectively)
elif [[ "$distro" == "Arch Linux" || "$distro" == "Manjaro" || "$distro" == "Garuda Linux" ]]; then
  echo "installing from pacman..."
  sudo pacman -S docker-compose
  # if the distribution is openSUSE
elif [[ "$distro" == "openSUSE" ]]; then
  echo "installing from zypper..."
  sudo zypper install docker docker-compose docker-compose-switch
fi

# downloading the docker file
echo Downloading Nextcloud AllInOne docker file...
curl https://raw.githubusercontent.com/CJBreen/Selfhosting-Toolkit/refs/heads/main/docker/compose.yaml -o ~/SelfService/Nextcloud/compose.yaml
sleep 1.5
echo Done.

# running the docker file itself
printf "Would you like to start up Nextcloud? [y/n] "
read answer
if [[ "$answer" != "${answer#[Yy]}" ]]; then
  docker compose -f ~/SelfService/Nextcloud/compose.yaml up -d
  echo Access Nextcloud by typing "localhost::8443" in a web browser.
else
  echo Exiting...
fi
