use crate::syntax::ast::{Pattern, Regex};

// SYNTAX
//
// regex ::= pattern
// pattern ::= alt-pattern
//
// alt-pattern ::= concat-pattern
//               | concat-pattern '|' alt-pattern
// concat-pattern ::= star-pattern
//                  | star-pattern concat-pattern
// star-pattern ::= primary-pattern
//                | primary-pattern '*'
// primary-pattern ::= <empty>
//                   | <non-meta-character>
//                   | '(' pattern ')'
pub fn parse(s: &str) -> Result<Regex, String> {
    let mut parser = Parser::new(s.as_bytes());
    parser.parse_regex()
}

struct Parser<'input> {
    str: &'input [u8],
    pos: usize,
}

impl<'input> Parser<'input> {
    fn new(s: &'input [u8]) -> Self {
        Self { str: s, pos: 0 }
    }

    fn parse_regex(&mut self) -> Result<Regex, String> {
        let p = self.parse_pattern()?;
        if self.pos == self.str.len() {
            Ok(Regex { root: p })
        } else {
            Err(format!("unconsumed input: {}", self.pos))
        }
    }

    fn parse_pattern(&mut self) -> Result<Box<Pattern>, String> {
        self.parse_alt_pattern()
    }

    fn parse_alt_pattern(&mut self) -> Result<Box<Pattern>, String> {
        let mut p1 = self.parse_concat_pattern()?;
        loop {
            if matches!(self.str.get(self.pos), Some(b'|')) {
                self.pos += 1;
                let p2 = self.parse_concat_pattern()?;
                p1 = Box::new(Pattern::Alt(p1, p2));
            } else {
                return Ok(p1);
            }
        }
    }

    fn parse_concat_pattern(&mut self) -> Result<Box<Pattern>, String> {
        let mut p1 = self.parse_star_pattern()?;
        loop {
            match self.str.get(self.pos) {
                None | Some(b'|') | Some(b')') => return Ok(p1),
                _ => {
                    let p2 = self.parse_star_pattern()?;
                    p1 = Box::new(Pattern::Concat(p1, p2));
                }
            }
        }
    }

    fn parse_star_pattern(&mut self) -> Result<Box<Pattern>, String> {
        let pat = self.parse_primary_pattern()?;
        if matches!(self.str.get(self.pos), Some(b'*')) {
            self.pos += 1;
            Ok(Box::new(Pattern::Star(pat)))
        } else {
            Ok(pat)
        }
    }

    fn parse_primary_pattern(&mut self) -> Result<Box<Pattern>, String> {
        match self.str.get(self.pos) {
            Some(b'(') => {
                self.pos += 1;
                let pat = self.parse_pattern()?;
                if matches!(self.str.get(self.pos), Some(b')')) {
                    self.pos += 1;
                    Ok(pat)
                } else {
                    Err("paren not balanced".into())
                }
            }
            Some(b')') => Ok(Box::new(Pattern::Empty)),
            Some(c) => {
                self.pos += 1;
                Ok(Box::new(Pattern::Literal(*c)))
            }
            None => Ok(Box::new(Pattern::Empty)),
        }
    }
}
