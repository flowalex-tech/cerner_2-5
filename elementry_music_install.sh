#!/bin/bash

# cerner_2tothe5th_2021


git clone https://github.com/elementary/music.git

cd music 

sudo apt-get install meson -y

sudo apt-get install libaccounts-glib-dev -y

sudo apt-get install libclutter-gtk-1.0-dev -y

sudo apt-get install libgda-5.0-dev -y

sudo apt-get install libgee-0.8-dev -y

sudo apt-get install  libglib2.0-dev -y

sudo apt-get install  libgpod-dev -y

sudo apt-get install  libgranite-dev -y

sudo apt-get install  libgsignon-glib-dev -y

sudo apt-get install  libgstreamer1.0-dev -y

sudo apt-get install  libgstreamer-plugins-base1.0-dev -y

sudo apt-get install  libgtk-3-dev -y

sudo apt-get install  libjson-glib-dev -y

sudo apt-get install  libnotify-dev -y

sudo apt-get install  libpeas-dev -y

sudo apt-get install  libsoup2.4-dev -y

sudo apt-get install  libtagc0-dev -y

sudo apt-get install libxml2-dev -y

sudo apt-get install  libzeitgeist-2.0-dev -y

sudo apt-get install  valac -y

cd music/
    
meson build --prefix=/usr

cd build

ninja

sudo ninja install
