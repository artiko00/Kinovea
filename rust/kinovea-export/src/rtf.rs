//! Plain text of an RTF comment, as `RichTextBox.Text` returns it after `RichTextBox.Rtf = comment`.

#[cfg(test)]
#[path = "rtf_test.rs"]
mod rtf_test;

/// Destinations whose whole group is dropped.
const IGNORED_DESTINATIONS: [&str; 4] = ["fonttbl", "colortbl", "stylesheet", "info"];

/// Windows-1252 characters for bytes 0x80..=0x9F. Other bytes are Latin-1 code points.
const CP1252_HIGH: [char; 32] = [
    '\u{20ac}', '\u{81}', '\u{201a}', '\u{192}', '\u{201e}', '\u{2026}', '\u{2020}', '\u{2021}',
    '\u{2c6}', '\u{2030}', '\u{160}', '\u{2039}', '\u{152}', '\u{8d}', '\u{17d}', '\u{8f}',
    '\u{90}', '\u{2018}', '\u{2019}', '\u{201c}', '\u{201d}', '\u{2022}', '\u{2013}', '\u{2014}',
    '\u{2dc}', '\u{2122}', '\u{161}', '\u{203a}', '\u{153}', '\u{9d}', '\u{17e}', '\u{178}',
];

#[derive(Clone, Copy)]
struct Group {
    skip: bool,
    uc: usize,
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
    groups: Vec<Group>,
    out: String,
}

/// Returns the plain text of an RTF document. Paragraph and line breaks become `\n`, but a
/// trailing `\n` is dropped: RichEdit keeps the final paragraph mark out of `RichTextBox.Text`.
pub fn rtf_to_text(rtf: &str) -> String {
    let mut parser = Parser {
        chars: rtf.chars().collect(),
        pos: 0,
        groups: vec![Group { skip: false, uc: 1 }],
        out: String::new(),
    };
    parser.run();
    let text = parser.out.as_str();
    text.strip_suffix('\n').unwrap_or(text).to_string()
}

impl Parser {
    fn run(&mut self) {
        while let Some(c) = self.bump() {
            match c {
                '{' => self.open_group(),
                '}' => self.close_group(),
                '\\' => self.control(),
                // Raw CR and LF in the RTF source are not text.
                '\r' | '\n' => {}
                _ => self.emit(c),
            }
        }
    }

    fn group(&mut self) -> &mut Group {
        self.groups.last_mut().expect("root group is never popped")
    }

    fn open_group(&mut self) {
        let group = *self.group();
        self.groups.push(group);
    }

    fn close_group(&mut self) {
        if self.groups.len() > 1 {
            self.groups.pop();
        }
    }

    fn emit(&mut self, c: char) {
        if !self.group().skip {
            self.out.push(c);
        }
    }

    fn control(&mut self) {
        let Some(c) = self.bump() else { return };
        match c {
            '\'' => {
                if let Some(code) = self.hex_pair() {
                    self.emit(cp1252(code));
                }
            }
            '*' => self.group().skip = true,
            '\\' | '{' | '}' => self.emit(c),
            c if c.is_ascii_alphabetic() => self.word(c),
            _ => {}
        }
    }

    fn word(&mut self, first: char) {
        let mut name = first.to_string();
        while let Some(c) = self.peek().filter(|c| c.is_ascii_alphabetic()) {
            name.push(c);
            self.pos += 1;
        }
        let param = self.number();
        if self.peek() == Some(' ') {
            self.pos += 1;
        }
        match name.as_str() {
            "par" | "line" => self.emit('\n'),
            "tab" => self.emit('\t'),
            "uc" => self.group().uc = usize::try_from(param.unwrap_or(1)).unwrap_or(0),
            "u" => {
                if let Some(code) = param {
                    self.unicode(code);
                }
            }
            dest if IGNORED_DESTINATIONS.contains(&dest) => self.group().skip = true,
            _ => {}
        }
    }

    fn number(&mut self) -> Option<i64> {
        let start = self.pos;
        if self.peek() == Some('-') {
            self.pos += 1;
        }
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 1;
        }
        let text: String = self.chars[start..self.pos].iter().collect();
        text.parse().ok()
    }

    fn hex_pair(&mut self) -> Option<u8> {
        let high = self.bump()?.to_digit(16)?;
        let low = self.bump()?.to_digit(16)?;
        Some((high * 16 + low) as u8)
    }

    fn unicode(&mut self, code: i64) {
        let code = if code < 0 { code + 65536 } else { code };
        let c = u32::try_from(code)
            .ok()
            .and_then(char::from_u32)
            .unwrap_or('\u{fffd}');
        self.emit(c);
        self.skip_fallback();
    }

    /// Skips the `\uc` characters that follow `\uN`. A `\'hh` escape counts as one.
    fn skip_fallback(&mut self) {
        let mut left = self.group().uc;
        while left > 0 {
            match self.peek() {
                Some('\\') if self.chars.get(self.pos + 1) == Some(&'\'') => self.pos += 4,
                Some('\\' | '{' | '}') | None => break,
                Some(_) => self.pos += 1,
            }
            left -= 1;
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += 1;
        Some(c)
    }
}

fn cp1252(code: u8) -> char {
    match code {
        0x80..=0x9f => CP1252_HIGH[usize::from(code - 0x80)],
        _ => char::from(code),
    }
}
