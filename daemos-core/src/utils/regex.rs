use color_eyre::Result;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct RegexExtract {
    re: Regex,
    group_position: usize,
}

impl RegexExtract {
    pub fn new(pattern: String, group_position: usize) -> Result<Self> {
        let re = Regex::new(&pattern)?;

        Ok(Self { re, group_position })
    }

    pub fn extract(&self) -> (String, usize) {
        (self.re.as_str().to_string(), self.group_position)
    }

    pub fn group_position(&self) -> usize {
        self.group_position
    }

    pub fn extract_group(&self, text: &str) -> Option<String> {
        let captures = self.re.captures(text)?;

        captures
            .get(self.group_position)
            .map(|m| m.as_str().to_string())
    }
}
