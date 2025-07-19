#[derive(Debug)]
pub enum Instr {
    Char(u8),
    Match,
    Jmp(usize),
    Split(usize, usize),
}
