//!

use processors::string_utils::StringUtils;

/**

*/
pub struct ProcessForTestLines {
}

impl ProcessForTestLines {

    /**
        we have a list of strings from input (like stdin).
        we are looking only for lines that fit this pattern
        test tests::failing::failing_one ... FAILED
        - or -
        test tests::passing::pass_one ... ok
    */
    pub fn find_test_lines(input : Vec<String>) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();

        for line in input {
            let left: String = line.from_left(4);
            
            if "test" == left {
                results.push(line);
            }
        }

        return  results;
    }
}