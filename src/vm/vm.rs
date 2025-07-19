use crate::vm::instr::Instr;

pub fn run(instrs: &[Instr], s: &[u8]) -> bool {
    let mut vm = Vm::new(instrs, s);
    vm.run()
}

struct Thread {
    pc: usize, // program counter
    sp: usize, // string pointer
}

impl Thread {
    fn new(pc: usize, sp: usize) -> Self {
        Self { pc, sp }
    }
}

struct Vm<'instr, 'input> {
    instrs: &'instr [Instr],
    string: &'input [u8],
    thread_stack: Vec<Thread>,
}

impl<'instr, 'input> Vm<'instr, 'input> {
    fn new(instrs: &'instr [Instr], string: &'input [u8]) -> Self {
        let initial_thread = Thread::new(0, 0);
        Self {
            instrs,
            string,
            thread_stack: vec![initial_thread],
        }
    }

    fn run(&mut self) -> bool {
        while let Some(t) = self.thread_stack.pop() {
            let mut pc = t.pc;
            let mut sp = t.sp;
            loop {
                match self.instrs[pc] {
                    Instr::Char(expected) => {
                        if let Some(c) = self.string.get(sp) {
                            if *c == expected {
                                pc += 1;
                                sp += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    Instr::Match => {
                        return true;
                    }
                    Instr::Jmp(a) => {
                        pc = a;
                    }
                    Instr::Split(a, b) => {
                        self.thread_stack.push(Thread::new(b, sp));
                        pc = a;
                    }
                }
            }
        }
        false
    }
}
