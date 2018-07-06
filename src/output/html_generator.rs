//! generates an html page using OrganizedTestResults (the output of cargo test)
//! http://siciarz.net/24-days-rust-tera/

use std::fs::File;
use std::io::Write;

use tera::{Context, Tera};

use output::output_trait::OutputTrait;
use processors::individual_test_results::IndividualTestResults;
use processors::test_set::{TestSetFunctions, TestSetCollection};
use processors::types::OrganizedTestResults;
use shell::factory::{ShellFactory, ShellTypes};
use utils::environment::Environment;
use utils::random::rand_string;



/**
    Generator for an html page using OrganizedTestResults (the output of cargo test)
*/
pub struct HtmlOutput {

}

impl OutputTrait for HtmlOutput {
    /**
        converts OrganizedTestResults into html using the template at env.template_dir
        (which should be the same as in src/templates directory)
    */
    fn generate(&self, test_results : &OrganizedTestResults) -> ShellTypes {

        let env :Environment = Environment::new();
        let template_search: String = format!("{}{}", env.template_dir, "/**/*");

        trace!("templates expected at: '{}'", template_search);

        let tera: Tera = Tera::new(&template_search).unwrap();
        let mut context: Context = Context::new();

        context.add("title", &"test results");
        context.add("from_location", &env.working_dir);

        let mut warning_msg_text : String = str2string!("");
        let mut raw_data : Vec<String> = Vec::new();

        if 0 == test_results.skipped.len()
            && 0 == test_results.success.len()
            && 0 == test_results.failed.len() {
            warning_msg_text = str2string!("No information was found to create report.");
            raw_data = test_results.raw_data.to_vec();
            raw_data.push(str2string!(""));
            raw_data.push(str2string!(">>> end of additional data"));
        }

        context.add("warning_message", &warning_msg_text);
        context.add("raw_data", &raw_data);

        trace!("skipped test count {}", test_results.skipped.len());
        context.add("ignored_tests",&test_results.skipped);

        trace!("success test count {}", test_results.success.len());
        context.add("successful_tests",&test_results.success);

        trace!("failed test count {}", test_results.failed.len());
        context.add("failed_tests",&test_results.failed);
        let rendered = tera.render("index.html", &context).expect("Failed to render template");

        trace!("results expected at: '{}'", env.results_dir);

        let html_file: String = format!("{}/{}.html", env.results_dir, rand_string(8));
        let mut f = File::create(html_file.to_string()).expect("Unable to create file");
        f.write_all(rendered.as_bytes()).expect("Unable to write data");

        return ShellFactory::get(&"browser" , &html_file);
    }
}

#[cfg(test)]
mod html_generator_tests {

    #[test]
    fn broken_code() {
        // hack
    }
}