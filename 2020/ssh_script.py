#!/usr/bin/env python3
# cerner_2^5_2020
#ssh to a host and run a specified comand

from os.path import join
from pexpect import pxssh
import getpass
import os
from pip._vendor.distlib.compat import raw_input

try:
    ssh_config = pxssh.pxssh()
    #Getting host and user info
    hostname = raw_input('Hostname: ')
    username = raw_input('Username: ')
    password = getpass.getpass('Password: ')

    ssh_config.login(hostname, username, password)

    ssh_config.sendline('hostname')

    ssh_config.prompt() #Match the prompt

    print( ssh_config.before) # Print everything before the prompt

    ssh_config.logout()
except pxssh.ExceptionPxssh as e:
    print('PXSSH failed to login')
    print(str(e))
