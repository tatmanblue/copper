#!/usr/bin/env bash
# this script is meant for developers needing to update their enviroment of changes
# and not wanting to use symbolic links

cp src/templates/index.html ~/.copper/templates/
rm ~/.copper/results/*
