#!/bin/bash


# cerner_2tothe5th_2021
git clone https://github.com/elementary/code.git

sudo apt-get install libeditorconfig-dev -y

sudo apt-get install libgee-0.8-dev -y

sudo apt-get install libgtksourceview-3.0-dev -y

sudo apt-get install libgtkspell3-3-dev -y

sudo apt-get install libpeas-dev -y

sudo apt-get install libsoup2.4-dev -y

sudo apt-get install libvte-2.91-dev -y

sudo apt-get install libvala-0.36-dev -y

sudo apt-get install libwebkit2gtk-4.0-dev -y

sudo apt-get install libzeitgeist-2.0 -y

cd code

meson build --prefix=/usr
cd build
ninja test

sudo ninja install
io.elementary.code
