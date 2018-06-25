//!

///  for a str, this trait adds string manipulation functions
///  to ease the work of dealing with strings
pub trait StringUtils {
    fn from_right(&self, count : usize) -> String;
    fn from_left(&self, count: usize) -> String;
    fn trimmed(&self) -> String;
}

/// this impl adds StringUtils to any string
impl StringUtils for String {

    fn from_right(&self, count : usize) -> String {
        let len = self.chars().count();

        if len < count {
            let complete: String = self.chars().collect::<String>();
            return complete.to_owned();
        }

        let right_most: String = self.chars()
                                    .skip(len - count).take(count)
                                    .collect::<String>();

        return right_most.to_owned();
    }

    fn from_left(&self, count: usize) -> String {
       let len = self.chars().count();

        if len < count {
            return self.to_string().to_owned();
        }

        let left_most: String = self.chars().take(count).collect::<String>();

        return left_most.to_owned();
    }

    fn trimmed(&self) -> String {
        return self.to_string().trim().to_string();
    }
}

#[cfg(test)]
mod string_utils_tests {

    use super::StringUtils;

    #[test]
    fn left_four_string_more_than_four_len() {
        let test: String = "123457890".to_string();

        let left: String = test.from_left(4);

        assert_eq!(left, "1234".to_string());
    }

    #[test]
    fn left_four_string_less_than_four_len() {
        let test: String = "123".to_string();

        let left: String = test.from_left(4);

        assert_eq!(left, test);

    }

    #[test]
    fn left_four_string_is_four_len() {
        let test: String = "1234".to_string();

        let left: String = test.from_left(4);

        assert_eq!(left, test);
    }

    #[test]
    fn string_is_empty() {
        let test: String = "".to_string();

        let left: String = test.from_left(4);

        assert_eq!(left, test);
    }

    #[test]
    fn left_one_string() {
        let test: String = "12345678901234567890".to_string();

        let left: String = test.from_left(1);

        assert_eq!(left, "1".to_string());
    }

    #[test]
    fn trimmed_when_string_is_empty() {
        let test: String = "".to_string();

        let left: String = test.trimmed();

        assert_eq!(left, test);
    }

    #[test]
    fn trimmed_when_theres_no_spaces() {
        let test: String = "blah blah blah".to_string();

        let left: String = test.trimmed();

        assert_eq!(left, test);
    }

    #[test]
    fn trimmed_when_spaces_at_right() {
        let test: String = "blah blah blah  ".to_string();

        let left: String = test.trimmed();

        assert_eq!(left, "blah blah blah".to_string());
    }

    #[test]
    fn trimmed_when_spaces_at_left() {
        let test: String = "     blah blah blah".to_string();

        let left: String = test.trimmed();

        assert_eq!(left, "blah blah blah".to_string());
    }

    #[test]
    fn trimmed_when_spaces_at_both() {
        let test: String = "     blah blah blah    ".to_string();

        let left: String = test.trimmed();

        assert_eq!(left, "blah blah blah".to_string());
    }
}