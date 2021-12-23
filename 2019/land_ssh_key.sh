#!/bin/bash
# cerner_2^5_2019

echo -n 'Please enter your password: '
read -sr USER_PASSWORD

#iterating through a file generated in some manner either manually or though some automation and copy you ssh key onto the remote hosts
cat hosts.txt | while read LINE
do
  sshpass -p "$USER_PASSWORD" ssh-copy-id $LINE
done
