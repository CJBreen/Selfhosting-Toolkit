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
