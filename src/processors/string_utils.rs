//!

///  for a str, this trait adds string manipulation functions
///  to ease the work of dealing with strings
pub trait StringUtils {
    fn from_right(&self, count : usize) -> String;
    fn from_left(&self, count: usize) -> String;
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
}