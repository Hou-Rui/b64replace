use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use regex::bytes::Regex;
use std::io::{BufRead, Write};

const BASE64_REGEX: &str = r"(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?";

#[derive(Debug)]
pub struct Base64Replacer {
    template: String,
}

#[derive(Debug, Clone)]
pub struct ReplaceEvent {
    /// Byte range in original line
    pub original_start: usize,
    pub original_end: usize,
    /// Byte range in resulting line
    pub replaced_start: usize,
    pub replaced_end: usize,
}

fn decode_base64_utf8(s: &[u8]) -> Result<String> {
    let bytes = STANDARD.decode(s)?;
    Ok(String::from_utf8(bytes)?)
}

fn read_next_line<R>(reader: &mut R, line: &mut Vec<u8>) -> bool
    where R: BufRead,
{
    match reader.read_until(b'\n', line) {
        Err(_) => false,
        Ok(0) => false,
        Ok(_) => true,
    }
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

    pub fn replace<R, W, F>(&self, reader: &mut R, writer: &mut W, callback: &F) -> Result<()>
    where
        R: BufRead,
        W: Write,
        F: Fn(&ReplaceEvent),
    {
        let re = self.build_regex()?;
        let mut line = vec![];
        let mut out = vec![];
        while read_next_line(reader, &mut line) {
            let mut last = 0;
            for cap in re.captures_iter(&line) {
                let matched = cap.get(0).unwrap();
                let encoded = cap.name("data").unwrap();
                if let Ok(decoded_str) = decode_base64_utf8(encoded.as_bytes()) {
                    out.extend(&line[last..encoded.start()]);
                    let replaced_start = out.len();
                    out.extend(decoded_str.as_bytes());
                    let replaced_end = out.len();
                    out.extend(&line[encoded.end()..matched.end()]);
                    let event = ReplaceEvent {
                        original_start: encoded.start(),
                        original_end: encoded.end(),
                        replaced_start,
                        replaced_end,
                    };
                    callback(&event);
                } else {
                    out.extend(matched.as_bytes());
                }
                last = matched.end();
            }
            
            out.extend(&line[last..]);
            writer.write(&out)?;
            line.clear();
            out.clear();
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
