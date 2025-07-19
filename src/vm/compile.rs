use crate::syntax::ast::{Pattern, Regex};
use crate::vm::instr::Instr;

pub fn compile(re: &Regex) -> Vec<Instr> {
    let compiler = Compiler::default();
    compiler.compile(&re.root)
}

#[derive(Default)]
struct Compiler {
    instrs: Vec<Instr>,
    labels: Vec<usize>,
}

impl Compiler {
    fn compile(mut self, pat: &Pattern) -> Vec<Instr> {
        self.compile_pattern(pat);
        self.resolve_labels();
        self.instrs.push(Instr::Match);
        self.instrs
    }

    fn compile_pattern(&mut self, pat: &Pattern) {
        match pat {
            Pattern::Empty => todo!(),
            Pattern::Literal(c) => self.instrs.push(Instr::Char(*c)),
            Pattern::Concat(a, b) => {
                self.compile_pattern(&a);
                self.compile_pattern(&b);
            }
            Pattern::Alt(a, b) => {
                //     split l1 l2
                // l1: compile(a)
                //     jmp l3
                // l2: compile(b)
                // l3:
                let label_1_index = self.create_new_label();
                let label_2_index = self.create_new_label();
                let label_3_index = self.create_new_label();
                self.instrs.push(Instr::Split(label_1_index, label_2_index));
                self.fix_label_address(label_1_index);
                self.compile_pattern(&a);
                self.instrs.push(Instr::Jmp(label_3_index));
                self.fix_label_address(label_2_index);
                self.compile_pattern(&b);
                self.fix_label_address(label_3_index);
            }
            Pattern::Star(a) => {
                // l1: split l2 l3
                // l2: compile(a)
                //     jmp l1
                // l3:
                let label_1_index = self.create_new_label();
                let label_2_index = self.create_new_label();
                let label_3_index = self.create_new_label();
                self.fix_label_address(label_1_index);
                self.instrs.push(Instr::Split(label_2_index, label_3_index));
                self.fix_label_address(label_2_index);
                self.compile_pattern(&a);
                self.instrs.push(Instr::Jmp(label_1_index));
                self.fix_label_address(label_3_index);
            }
        }
    }

    fn resolve_labels(&mut self) {
        for instr in self.instrs.iter_mut() {
            match instr {
                &mut Instr::Jmp(ref mut a) => {
                    *a = self.labels[*a];
                }
                &mut Instr::Split(ref mut a, ref mut b) => {
                    *a = self.labels[*a];
                    *b = self.labels[*b];
                }
                _ => (),
            }
        }
    }

    fn create_new_label(&mut self) -> usize {
        self.labels.push(0);
        self.labels.len() - 1
    }

    fn fix_label_address(&mut self, label_index: usize) {
        let address = self.instrs.len();
        self.labels[label_index] = address;
    }
}
