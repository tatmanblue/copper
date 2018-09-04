# Copper
**C**argo Test **O**utput **P**arser.

This application provides a pretty UI to "cargo test" output. 

Cargo test output is written to the console.  You can redirect it to a file but it really isn't any more readable.  This project provides an easy to read and easy to navigate web page for reviewing cargo test results.

[![Build Status](https://travis-ci.org/mattraffel/copper.svg?branch=master)](https://travis-ci.org/mattraffel/copper)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/mattraffel/copper#license)

# Installation
The following steps are required prior to running:
1) create a directory ".copper" in your $HOME folder.
2) create two subdirectores in ".copper": templates, results
3) copy src/templates into ".copper/templates"
4) build project using standard cargo commands
5) (optional) create a symbolic link to copper binary in your path.


# Running
The simplest method is to direct output from "cargo test" to copper. eg:
> (another project dir) cargo test | copper
## Please note: 
Do not use --no-capture.  The output is different. Copper currently works with output from cargo test only.


# License
Released under Apache 2.0 and MIT.  See license files in git repo.