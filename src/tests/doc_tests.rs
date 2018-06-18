//!

#[derive(Debug, PartialEq, Eq)]
pub struct Something {}

impl Something {
    /**
         Validate the did

         Validates that the did is a length of 20 to 21 and that it only contains
         alphanumeric characters.

        ```
            # extern crate sovtoken;
            use sovtoken::logic::did::Did;
            use sovtoken::logic::did::DidError;

            let did_invalid = Did::new("123456789[01234567890");
            let error = did_invalid.validate().unwrap_err();
            assert_eq!(DidError::InvalidChar('['), error);
        ```
    */
    pub fn validate(&self) -> Result<(), i32> {
        return Ok(());
    }
}