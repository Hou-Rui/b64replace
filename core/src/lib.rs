use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use regex::bytes::{Captures, Regex};
use std::io::{BufRead, Write};

const BASE64_REGEX: &str = r"(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?";

pub struct Base64Replacer {
    template: String,
}

fn decode_base64_utf8(s: &String) -> Result<String> {
    let bytes = STANDARD.decode(s)?;
    Ok(String::from_utf8(bytes)?)
}

impl Base64Replacer {
    pub fn new(template: String) -> Self {
        Self {
            template: if template.is_empty() {
                String::from("^{}$")
            } else {
                template
            },
        }
    }

    pub fn replace_all<R, W>(&mut self, reader: &mut R, writer: &mut W) -> Result<()>
    where
        R: BufRead,
        W: Write,
    {
        let re = self.build_regex()?;
        let mut buffer = Vec::new();
        while let Ok(bytes) = reader.read_until(b'\n', &mut buffer) {
            if bytes == 0 {
                break;
            }
            let out = re.replace_all(&buffer, |caps: &Captures| {
                let full = String::from_utf8_lossy(&caps[0]).to_string();
                let encoded = String::from_utf8_lossy(&caps["data"]).to_string();
                match decode_base64_utf8(&encoded) {
                    Ok(decoded) => full.replace(&encoded, &decoded),
                    Err(_) => full, // leave unchanged on failure
                }
            });
            writer.write(&out)?;
            buffer.clear();
        }
        Ok(())
    }

    fn build_regex(&self) -> Result<Regex> {
        let parts: Vec<&str> = self.template.split("{}").collect();
        if parts.len() != 2 {
            anyhow::bail!("Template must contain exactly one {{}} placeholder!");
        }
        let pattern = format!("{}(?P<data>{}){}", parts[0], BASE64_REGEX, parts[1]);
        Ok(Regex::new(&pattern)?)
    }
}
