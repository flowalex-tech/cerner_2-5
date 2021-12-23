#!/bin/bash
# cerner_2^5_2020
echo "Script will install Drupal using a Docker Container"
#The following command is setting up the database, please remeber to change the database name which currently is set as MYSQL-NAME and the root password which is MYSQL-PASSWORD
#docker run --name MYSQL-NAME -e MYSQL_ROOT_PASSWORD=MYSQL-PASSWORD -d mysql:latest
read -rp "Please enter a name for your database: " MYSQLNAME
read -srp "Please enter a password for your database: " MYSQLPASSWORD
read -rp "Please enter the verion you want of the MYSQL docker image: " MYSQL_VERSION
docker run --name $MYSQLNAME -e MYSQL_ROOT_PASSWORD=$MYSQLPASSWORD -d hypriot/rpi-mysql:$MYSQL_VERSION
read -rp "Please enter a name for your Drupal Container: " somedrupal
read -rp "Please enter the verion you want of the MYSQL docker image: " DRUPAL_VERSION
docker run --name $somedrupal --link $MYSQLNAME:mysql -p 8080:80 -e MYSQL_USER=root -e MYSQL_PASSWORD=$MYSQLPASSWORD -d drupal:$DRUPAL_VERSION
clear
echo "Please record this information as it will be used later for installing the Drupal Site"
echo "Your site Docker Container iformation is:
MYSQL Container Name: $MYSQLNAME"
read -p "Do you want to see the username/password for the MYSQL database? y/n: " -n 1 -r
if [[ $REPLY =~ ^[Yy]$ ]]
then
    echo "MYSQL USERNAME: root
    MYSQLPASSWORD: $MYSQLPASSWORD"
fi
echo "Drupal Container Name: $somedrupal"
echo " When setting up the database for the drupal site enter the following information:
Database name: drupal
Database Host: $MYSQLNAME"
read -p "Do you want to see the username/password for the MYSQL database? y/n: " -n 1 -r
if [[ $REPLY =~ ^[Yy]$ ]] 
then
    echo
    echo "MYSQL USERNAME: root
    MYSQLPASSWORD: $MYSQLPASSWORD"
fi
