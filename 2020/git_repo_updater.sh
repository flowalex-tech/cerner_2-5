#!/bin/bash
# cerner_2^5_2020
# Simple bash script that updates your fork off an upstream repo and pulls from the current branch you are one
# shellcheck disable=SC2086
#Sets the branch to your current branch.
branch=$(git symbolic-ref --short HEAD)
#Displays remotes, fetches all branches and then pulls from the upstream remote into the branch you are currently on
git remote -v && git fetch --all && git pull upstream $branch
