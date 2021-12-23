#!/bin/sh
#cerner_2^5_2019
#   Creates a drupal site on a lamp stack (assumes that apache, php-7 and mysql are already installed) Needs to have the drupal version updated, a manual process.
#    This script will install a Drupal 8 site running 8.4.2
# Script installing themes and a version for installing drupal commerce can be found here: https://gitlab.com/flowalex/DrupalInstallScript/tree/master
#This moves the script into the home folder to download and expand the script
cd $HOME 
#This command downloads Drupal 8.3.2 to update the script to the newer version of drupal when released go to drupal.org and navitage to https://www.drupal.org/download and right click the download drupal 8.X.X
wget https://ftp.drupal.org/files/projects/drupal-8.4.2.tar.gz
#This unzips the drupal folder, if using a later version of drupal please change the drupal-8.X.X.tar.gz to match your version
tar xzvf drupal-8.4.2.tar.gz
#Moves the contents of the folder to the workspace directory 
mv drupal-8.4.2/* workspace/
mv drupal-8.4.2/.htaccess workspace/
#Moves to the workspace directory
cd $HOME/workspace
#Moves to the sites/default directory to create the settings.php file
cd sites/default/
cp default.settings.php settings.php
#changes the permissions of the settings.php file so that it can be written to during the install
chmod a+w settings.php
#makes the files directory and changes the permission so that files can be written during install
mkdir files
chmod a+w files/
#moves back into the workspace
cd $HOME/workspace
#installs mysql and starts it
mysql-ctl install
mysql-ctl start
mysql-ctl status
#installs phpmyadmin
phpmyadmin-ctl install




