#!/bin/bash
# cerner_2^5_2020

#clones the repo that needs to have an image built
git clone https://github.com/flowalex-tech/flask_microblog.git

cd flask_microblog || exit 1

#Builds the docker image (full tagging push command missing since I have other automation that handles image builds for me)
docker build . -t flask_microblog
