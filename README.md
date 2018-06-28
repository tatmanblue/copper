# Copper
**C**argo Test **O**utput **P**arser.

This application provides a pretty UI to "cargo test" output. 
[![Build Status](https://travis-ci.org/mattraffel/copper.svg?branch=master)](https://travis-ci.org/mattraffel/copper)


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
