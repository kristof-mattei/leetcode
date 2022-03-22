#!/bin/bash

git fetch upstream

git checkout -b update-from-upstream
git checkout update-from-upstream

git merge upstream/main
