pub struct Regex {
    pub root: Box<Pattern>,
}

pub enum Pattern {
    Empty,
    Literal(u8),
    Concat(Box<Pattern>, Box<Pattern>),
    Alt(Box<Pattern>, Box<Pattern>),
    Star(Box<Pattern>),
}
