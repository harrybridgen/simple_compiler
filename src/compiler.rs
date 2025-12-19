use crate::grammar::{AST, Instruction, Oper};

pub fn compile(ast: AST, code: &mut Vec<Instruction>, label_gen: &mut label_gen) {
    match ast {
        AST::Number(n) => code.push(Instruction::Push(n)),
        AST::Oper(left, oper, right) => {
            compile(*left, code, label_gen);
            compile(*right, code, label_gen);

            match oper {
                Oper::Addition => code.push(Instruction::Add),
                Oper::Division => code.push(Instruction::Div),
                Oper::Multiplication => code.push(Instruction::Mul),
                Oper::Subtraction => code.push(Instruction::Sub),
                Oper::Greater => code.push(Instruction::Greater),
                Oper::Less => code.push(Instruction::Less),
                Oper::Equal => code.push(Instruction::Equal),
                Oper::Or => code.push(Instruction::Or),
                Oper::And => code.push(Instruction::And),
            }
        }
        AST::Assign(name, ast) => {
            compile(*ast, code, label_gen);
            code.push(Instruction::Store(name));
        }
        AST::Var(name) => {
            code.push(Instruction::Load(name));
        }
        AST::Program(statements) => {
            for statement in statements {
                compile(statement, code, label_gen);
            }
        }
        AST::LazyAssign(name, ast) => {
            code.push(Instruction::StoreLazy(name, ast));
        }
        AST::Print(ast) => {
            compile(*ast, code, label_gen);
            code.push(Instruction::Print);
        }
        AST::IfElse(cond, ifbranch, elsebranch) => {
            compile(*cond, code, label_gen);

            let else_label = label_gen.fresh("else");
            let end_label = label_gen.fresh("end");

            code.push(Instruction::JumpIfZero(else_label.clone()));

            for ast in ifbranch {
                compile(ast, code, label_gen);
            }

            code.push(Instruction::Jump(end_label.clone()));

            code.push(Instruction::Label(else_label));

            for ast in elsebranch {
                compile(ast, code, label_gen);
            }

            code.push(Instruction::Label(end_label));
        }
    }
}

pub struct label_gen {
    counter: usize,
}

impl label_gen {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn fresh(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.counter);
        self.counter += 1;
        label
    }
}
