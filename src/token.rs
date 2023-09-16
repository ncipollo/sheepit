pub const VERSION: &str = "{version}";

#[derive(Debug, PartialEq)]
pub struct TokenTrimmer<'a> {
    pub before_token: &'a str,
    pub after_token: &'a str,
}

impl<'a> TokenTrimmer<'a> {
    pub fn new<'b>(text: &'b str, token: &str) -> Option<TokenTrimmer<'b>> {
        if token.is_empty() || text == token {
            return None;
        }

        let token_index = text.find(token)?;
        let after_token_index = token_index + token.len();
        let split = TokenTrimmer {
            before_token: &text[0..token_index],
            after_token: &text[after_token_index..],
        };
        Some(split)
    }

    pub fn trim_text<'b>(&self, text: &'b str) -> &'b str {
        let without_prefix = text.strip_prefix(self.before_token).unwrap_or(text);
        without_prefix.strip_suffix(self.after_token).unwrap_or(without_prefix)
    }
}

#[cfg(test)]
mod test {
    use crate::token::{TokenTrimmer};

    #[test]
    fn new_empty_text() {
        let result = TokenTrimmer::new("", "$token");
        assert_eq!(None, result)
    }

    #[test]
    fn new_empty_token() {
        let result = TokenTrimmer::new(" text ", "");
        assert_eq!(None, result)
    }

    #[test]
    fn new_no_token_found() {
        let result = TokenTrimmer::new("my string", "$token");
        assert_eq!(None, result)
    }

    #[test]
    fn new_token_at_start() {
        let result = TokenTrimmer::new("$token suffix", "$token");
        let expected = TokenTrimmer { before_token: "", after_token: " suffix" };
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn new_token_at_end() {
        let result = TokenTrimmer::new("prefix $token", "$token");
        let expected = TokenTrimmer { before_token: "prefix ", after_token: "" };
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn new_token_in_middle() {
        let result = TokenTrimmer::new("prefix $token suffix", "$token");
        let expected = TokenTrimmer { before_token: "prefix ", after_token: " suffix" };
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn new_only_token() {
        let result = TokenTrimmer::new("$token", "$token");
        assert_eq!(None, result)
    }

    #[test]
    fn trim_text_trims_suffix_and_prefix() {
        let token_trimmer = TokenTrimmer::new("prefix_$token_suffix", "$token")
            .expect("trimmer should exist");
        let trimmed_text = token_trimmer.trim_text("prefix_1.0.0_suffix");
        assert_eq!("1.0.0", trimmed_text)
    }

    #[test]
    fn trim_text_trims_prefix() {
        let token_trimmer = TokenTrimmer::new("prefix_$token", "$token")
            .expect("trimmer should exist");
        let trimmed_text = token_trimmer.trim_text("prefix_1.0.0_suffix");
        assert_eq!("1.0.0_suffix", trimmed_text)
    }

    #[test]
    fn trim_text_trims_prefix_no_suffix() {
        let token_trimmer = TokenTrimmer::new("prefix_$token", "$token")
            .expect("trimmer should exist");
        let trimmed_text = token_trimmer.trim_text("prefix_1.0.0");
        assert_eq!("1.0.0", trimmed_text)
    }

    #[test]
    fn trim_text_trims_suffix() {
        let token_trimmer = TokenTrimmer::new("$token_suffix", "$token")
            .expect("trimmer should exist");
        let trimmed_text = token_trimmer.trim_text("prefix_1.0.0_suffix");
        assert_eq!("prefix_1.0.0", trimmed_text)
    }

    #[test]
    fn trim_text_trims_no_prefix() {
        let token_trimmer = TokenTrimmer::new("$token_suffix", "$token")
            .expect("trimmer should exist");
        let trimmed_text = token_trimmer.trim_text("1.0.0_suffix");
        assert_eq!("1.0.0", trimmed_text)
    }
}