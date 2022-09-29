#!/bin/bash

git fetch origin
git fetch upstream

git branch -D update-from-upstream
git branch update-from-upstream origin/main
git checkout update-from-upstream

git merge upstream/main
