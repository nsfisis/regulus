use crate::syntax::ast::Regex;

pub mod compile;
pub mod instr;
pub mod vm;

pub fn is_match(re: Regex, s: &str) -> bool {
    let instrs = compile::compile(&re);
    vm::run(&instrs, s.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn re(p: &str) -> Regex {
        use crate::syntax::parse::parse;
        parse(p).unwrap()
    }

    #[test]
    fn matches() {
        assert!(is_match(re("a"), "a"));
        assert!(is_match(re("a*"), ""));
        assert!(is_match(re("a*"), "a"));
        assert!(is_match(re("a|b*"), "a"));
        assert!(is_match(re("a|b*"), "bb"));
        assert!(is_match(re("(a|b)*"), "abababaa"));
        assert!(is_match(re("a*b*"), "abb"));
    }

    #[test]
    fn does_not_match() {
        assert!(!is_match(re("a"), "b"));
    }
}
