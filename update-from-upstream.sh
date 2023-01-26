#!/bin/bash

# fetch so we can merge in without first
# having to checkout & pull the branches
git fetch origin
git fetch upstream

# delete existing branch
git branch -D update-from-upstream

# we create a new branch from the origin's main (=HEAD, because we fetched)
git branch update-from-upstream origin/main

# check out the branch
git checkout update-from-upstream

# merge in the changes from upstream
git merge upstream/main

# set the current branch's upstream to where we need it to be so we can
# push without being prompted
git branch --set-upstream-to=origin/update-from-upstream update-from-upstream

# git push origin update-from-upstream
