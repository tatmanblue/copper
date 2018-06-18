# rust-test-output-parser
Parser of rust unit test output (cargo test command)

# Installation
The following steps are required prior to running:
1) create a directory ".rust-test-parser" in your $HOME folder.
2) create two subdirectores in ".rust-test-parser": templates, results
3) copy src/templates into ".rust-test-parser/templates"
4) build project using standard cargo commands
5) (optional) create a symbolic link to rust-test-parser binary in your path.


# Running
The simplest method is to direct output from "cargo test" to rust-test-parser. eg:
> (another project dir) cargo test | rust-test-parser
