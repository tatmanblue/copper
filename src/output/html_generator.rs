//!  http://siciarz.net/24-days-rust-tera/

use output::output_trait::OutputTrait;
use processors::types::OrganizedTestResults;
use tera::{Context, Tera};


/**
*/
pub struct HtmlOutput {

}

impl OutputTrait for HtmlOutput {
    fn generate(test_results : &OrganizedTestResults) {

        let tera: Tera = Tera::new("templates/**/*").unwrap();
        let mut context: Context = Context::new();

        context.add("title", &"hello world!");
        context.add("content", &"stuff stuff stuff stuff");
        context.add("todos",
                &vec!["buy milk", "walk the dog", "write about tera"]);
        let rendered = tera.render("index.html", &context).expect("Failed to render template");
        println!("{}", rendered);


    }
}