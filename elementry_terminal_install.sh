#!/bin/bash

#old script, built elementry_os terminal on other operating systems
#cerner_2tothe5th_2021

git clone https://github.com/elementary/terminal.git
sleep 10s
sudo apt-get install libgranite-dev -y

sudo apt-get install libvte-2.91-dev -y

sudo apt-get install meson -y

sudo apt-get install valac -y


cd terminal

meson build --prefix=/usr

cd build
ninja test

sudo ninja install
io.elementary.terminal
