use regex::Regex;

pub type ConvertResult = Option<Vec<String>>;

pub trait SkkConverter {
    fn convert(&self, src: &str) -> ConvertResult;
}

pub struct Emanon001Converter {}

impl Emanon001Converter {
    pub fn new() -> Self {
        Self {}
    }
}

impl SkkConverter for Emanon001Converter {
    fn convert(&self, src: &str) -> ConvertResult {
        let re = Regex::new(r"\Aぼく\z").ok()?;
        if re.is_match(src) {
            Some(vec!["emanon001".into()])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skk_converter() {
        let converter = Emanon001Converter::new();
        assert_eq!(
            converter.convert("ぼく"),
            Some(vec!["emanon001".to_string()])
        );
        assert_eq!(converter.convert("わたし"), None);
    }
}
