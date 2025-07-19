pub mod ast;
pub mod parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid() {
        assert!(parse::parse("").is_ok());
        assert!(parse::parse("a").is_ok());
        assert!(parse::parse("ab").is_ok());
        assert!(parse::parse("abc").is_ok());
        assert!(parse::parse("a|b").is_ok());
        assert!(parse::parse("a|b*").is_ok());
        assert!(parse::parse("(a|b)*").is_ok());
        assert!(parse::parse("(((a|b)))*").is_ok());
        assert!(parse::parse("a*b*").is_ok());
    }

    #[test]
    fn parse_invalid() {
        assert!(parse::parse("(").is_err());
        assert!(parse::parse("()))").is_err());
        assert!(parse::parse("(((())").is_err());
    }
}
