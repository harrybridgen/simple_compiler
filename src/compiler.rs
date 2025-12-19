use crate::grammar::{AST, Instruction, Operator};

pub fn compile(ast: AST, code: &mut Vec<Instruction>, label_gen: &mut LabelGenerator) {
    match ast {
        AST::Number(n) => code.push(Instruction::Push(n)),
        AST::Operation(left, oper, right) => {
            compile(*left, code, label_gen);
            compile(*right, code, label_gen);

            match oper {
                Operator::Addition => code.push(Instruction::Add),
                Operator::Division => code.push(Instruction::Div),
                Operator::Multiplication => code.push(Instruction::Mul),
                Operator::Subtraction => code.push(Instruction::Sub),
                Operator::Greater => code.push(Instruction::Greater),
                Operator::Less => code.push(Instruction::Less),
                Operator::Equal => code.push(Instruction::Equal),
                Operator::Or => code.push(Instruction::Or),
                Operator::And => code.push(Instruction::And),
            }
        }

        AST::Loop(block) => {
            let looplabel = label_gen.fresh("loop");

            code.push(Instruction::Label(looplabel.clone()));
            for ast in block {
                compile(ast, code, label_gen);
            }
            code.push(Instruction::Jump(looplabel));
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

pub struct LabelGenerator {
    counter: usize,
}

impl LabelGenerator {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn fresh(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.counter);
        self.counter += 1;
        label
    }
}
