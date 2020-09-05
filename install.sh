#!/bin/bash
set -ex

sudo apt update && sudo apt build-dep opencv wget -y
mkdir deps
pushd deps
wget https://github.com/opencv/opencv/archive/4.3.0.tar.gz
tar zxvf 4.3.0.tar.gz
pushd opencv-4.3.0
mkdir build
pushd build
cmake ../ -D CMAKE_INSTALL_PREFIX=../installed
make
popd
popd

