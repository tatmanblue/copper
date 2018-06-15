//!  http://siciarz.net/24-days-rust-tera/

use std::fs::File;
use std::io::Write;

use tera::{Context, Tera};

use output::output_trait::OutputTrait;
use processors::types::OrganizedTestResults;
use utils::environment::Environment;
use utils::random::rand_string;



/**
*/
pub struct HtmlOutput {

}

impl OutputTrait for HtmlOutput {
    /**
        converts OrganizedTestResults into html using the template at env.template_dir
        (which should be the same as in src/templates directory)
    */
    fn generate(test_results : &OrganizedTestResults) {

        let env :Environment = Environment::new();
        let template_search: String = format!("{}{}", env.template_dir, "/**/*");

        trace!("templates expected at: '{}'", template_search);

        let tera: Tera = Tera::new(&template_search).unwrap();
        let mut context: Context = Context::new();

        context.add("title", &"test results");
        context.add("successful_tests",&test_results.success);
        context.add("ignored_tests",&test_results.skipped);
        context.add("failed_tests",&test_results.failed);
        let rendered = tera.render("index.html", &context).expect("Failed to render template");

        trace!("results expected at: '{}'", env.results_dir);

        let html_file: String = format!("{}/{}.html", env.results_dir, rand_string(8));
        let mut f = File::create(html_file).expect("Unable to create file");
        f.write_all(rendered.as_bytes()).expect("Unable to write data");
    }
}