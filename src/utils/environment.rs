//! Enviroment type is responsible for data that affects the running enviroment of copper

use std::path::Path;
use std::process::exit;
use std::env;

use ansi_term::*;

use utils::file_utils::FileUtilities;
use utils::string_utils::StringUtils;

/**
    Contains, finds and builds any environmental data the user controls for running
    copper
*/
pub struct Environment {
    /**
        directory where copper is invoked from
        aka . fully qualified
    */
    pub working_dir: String,
    /**
        expectation is this will be $HOME/.copper/templates
    */
    pub template_dir : String,
    /**
        expectation is this will be $HOME/.copper/results
    */
    pub results_dir : String,
    /**
        default is "default" or "html" aka web page is generated with the test results
    */
    pub output_format: String,
    /**
        default is false.  when true, output also appears in the console
    */
    pub include_console_format: bool,
    /**
    */
    pub read_from_file: bool,
    /**
    */
    pub input_file_name : String,
}

impl Environment {
    fn print_help() {
        println!();
        println!("{}", Color::Green.paint("copper"));
        println!("\t by matt raffel");
        println!();
        println!("usage is pretty simple, assuming copper is in your path. call it like this,");
        println!("in the same directory you ran {}:", Color::White.paint("cargo build"));
        println!("{}", Color::Blue.paint("\tcargo test | copper"));
        println!();
        println!("Some setup is required.  Please read the READ.ME");
        println!();
        println!("additional commands include:");
        println!("    -d, --delete       clean up working folders");
        println!("    -f, --file <FILE>  get test results from file not stdin");
        println!("                           eg: copper --file bob.txt");
    }

    fn get_env_working_dir() -> String {
        return env::current_dir().unwrap().to_str().unwrap().to_string();
    }

    fn get_working_dir() -> String {
        match env::current_dir() {
            Ok(path) => return path.to_str().unwrap().to_string(),
            Err(_e) => return ".".to_string(),
        }
    }

    /*
        should be $HOME/.copper
        if there is an error then current working directory
    */
    fn get_home_dir() -> String {
        match env::home_dir() {
            Some(path) => return format!("{}{}", path.to_str().unwrap().to_string(), "/.copper"),
            None => return Environment::get_working_dir(),
        }
    }

    /**
        Constructor
    */
    pub fn new() -> Environment {

        let working_dir: String = Environment::get_env_working_dir();
        let home_dir : String = Environment::get_home_dir();
        let template_dir : String = format!("{}{}", home_dir, "/templates");
        let results_dir : String = format!("{}{}", home_dir, "/results");
        let mut output_format: String = "default".to_string();
        let mut read_from_file: bool = false;
        let mut input_file_name: String = "".to_string();

        if true ==  Environment::has_file_name_parameter() {
            read_from_file = true;
            input_file_name = Environment::get_file_name_parameter();
        }

        if true == Environment::has_output_parameter() {
            output_format = Environment::get_output_parameter();
        }

        return Environment {
            working_dir,
            template_dir,
            results_dir,
            output_format,
            include_console_format: false,
            read_from_file,
            input_file_name
        };
    }

    /**
        static method, prints help and process exits if help is passed on the commandline
    */
    pub fn exit_if_print_help() {
        if env::args().find(|a| a == "-h" || a == "--help").is_some() {
            Environment::print_help();
            exit(0);
        }
    }

    /**
        static method, cleans up results folder and exits if -d or --delete is passed on command line
    */
    pub fn exit_on_clean_up() {
        if env::args().find(|a| a == "-d" || a == "--delete").is_some() {

            let environment = Environment::new();
            FileUtilities::clean_up_results(&environment.results_dir).unwrap();

            exit(0);
        }
    }

    /**
        checks for -f/--file in the command line input
    */
    pub fn has_file_name_parameter() -> bool {
        if env::args().find(|a| a == "-f" || a == "--file").is_some() {
            return true;
        }
        return false;
    }

    /**
        checks for -o/--output in the command line input
    */
    pub fn has_output_parameter() -> bool {
        if env::args().find(|a| a == "-o" || a == "--output").is_some() {
            return true;
        }
        return false;
    }

    /**
        returns output argument when -o/--output is in the command line input
    */
    pub fn get_output_parameter() -> String {
        if Environment::has_output_parameter() {
            let mut found_it : bool = false;

            for argument in env::args() {
                if true == found_it {
                    let left : String = argument.from_left(1);
                    if "-" != left {
                        return argument;
                    }
                }

                if "-o" == argument || "--output" == argument {
                    found_it = true;
                }
            }
        }

        panic!("expected a output format parameter");
    }

    /**
        returns filename argument when -f/--file is in the command line input
    */
    pub fn get_file_name_parameter() -> String {

        if env::args().find(|a| a == "-f" || a == "--file").is_some() {
            let mut found_it : bool = false;

            for argument in env::args() {
                if true == found_it {
                    let left : String = argument.from_left(1);
                    if "-" != left {
                        return argument;
                    }
                }

                if "-f" == argument || "--file" == argument {
                    found_it = true;
                }
            }
        }

        panic!("expected a file name in command args");
    }

    /**
        returns the directory name (only, no path) of working_dir
    */
    pub fn get_working_dir_short_name(&self) -> String {

        let dir = self.working_dir.to_string();
        let os_dir_name = Path::new(&dir).file_name().unwrap();
        let short_name = os_dir_name.to_str().unwrap();

        return short_name.to_string();
    }
}

#[cfg(test)]
mod environment_tests {
    use std::path::Path;
    use utils::environment::Environment;

    #[test]
    fn get_root_dir() {
        let env :Environment = Environment::new();
        let working_dir = env.working_dir;

        println!("working dir '{}'", working_dir);
        println!("short name is '{:?}'", Path::new(&working_dir).file_name().unwrap());


    }
}