use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use regex::{Captures, Regex};
use std::io::{BufRead, Write};

const BASE64_REGEX: &str = r"(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?";

pub struct Base64Replacer {
    template: String,
}

fn decode_base64_utf8(s: &str) -> Result<String> {
    let bytes = STANDARD.decode(s)?;
    Ok(String::from_utf8(bytes)?)
}

impl Base64Replacer {
    pub fn new(template: String) -> Self {
        Self { template }
    }

    pub fn replace_all<R, W>(&mut self, reader: R, writer: &mut W) -> Result<()>
    where
        R: BufRead,
        W: Write,
    {
        let re = self.build_regex()?;
        for line in reader.lines() {
            let line = line?;
            let out = re.replace_all(&line, |caps: &Captures| {
                let full = caps.get(0).unwrap().as_str();
                let encoded = &caps["data"];
                match decode_base64_utf8(encoded) {
                    Ok(decoded) => full.replace(encoded, &decoded),
                    Err(_) => full.to_string(), // leave unchanged on failure
                }
            });
            writeln!(writer, "{}", out.to_string())?;
        }
        Ok(())
    }

    fn build_regex(&self) -> Result<Regex> {
        let parts: Vec<&str> = self.template.split("{}").collect();
        if parts.len() != 2 {
            anyhow::bail!("template must contain exactly one {{}} placeholder");
        }
        let pattern = format!("{}(?P<data>{}){}", parts[0], BASE64_REGEX, parts[1]);
        Ok(Regex::new(&pattern)?)
    }
}
