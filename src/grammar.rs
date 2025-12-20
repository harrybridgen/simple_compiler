// program        ::= statement (";" statement)* ";"?

// statement      ::= assignment
//                  | array_assignment
//                  | reactive_assignment
//                  | immutable_assignment
//                  | if_statement
//                  | loop_statement
//                  | break_statement
//                  | print_statement
//                  | println_statement
//                  | expression

// assignment     ::= identifier "=" expression

// reactive_assignment
//                 ::= identifier "::=" expression

// immutable_assignment
//                 ::= identifier ":=" expression

// array_assignment
//                 ::= identifier "[" expression "]" "=" expression
//                  | identifier "[" expression "]" "::=" expression

// if_statement   ::= "if" expression block ("else" block)?

// loop_statement ::= "loop" block

// break_statement
//                 ::= "break"

// block          ::= "{" statement (";" statement)* ";"? "}"

// print_statement
//                 ::= "print" expression

// println_statement
//                 ::= "println" expression

// expression     ::= or_expr

// or_expr        ::= and_expr ("||" and_expr)*

// and_expr       ::= comparison ("&&" comparison)*

// comparison     ::= additive ((">" | "<" | ">=" | "<=" | "==" | "!=") additive)*

// additive       ::= multiplicative (("+" | "-") multiplicative)*

// multiplicative ::= postfix (("*" | "/") postfix)*

// postfix        ::= factor ("[" expression "]")*

// factor         ::= number
//                  | identifier
//                  | "-" factor
//                  | "(" expression ")"
//                  | "[" expression "]"     // array creation

// identifier     ::= [a-zA-Z][a-zA-Z0-9]*
// number         ::= [0-9]+

// comment        ::= "#" .* "#"


#[derive(Debug, Clone)]
pub enum Token {
    Add,
    Mul,
    Div,
    Sub,
    Number(i32),
    LParen,
    RParen,
    Ident(String),
    Assign,
    ReactiveAssign,
    ImmutableAssign,
    Semicolon,
    Print,
    Println,
    Greater,
    Less,
    Equal,
    Or,
    And,
    If,
    Else,
    LBrace,
    RBrace, LSquare, RSquare,
    Loop,
    Break,
    LessEqual,
    GreaterEqual,
    NotEqual,
    Not,
}

#[derive(Debug, Clone)]
pub enum Type {
    Integer(i32),
    LazyInteger(Box<AST>),Array(Vec<Type>),
}

#[derive(Debug, Clone)]
pub enum AST {
    Number(i32),
    Operation(Box<AST>, Operator, Box<AST>),
    Assign(String, Box<AST>),
    ReactiveAssign(String, Box<AST>),
    ImmutableAssign(String, Box<AST>),
    Var(String),
    Program(Vec<AST>),
    Print(Box<AST>),
    Println(Box<AST>),
    IfElse(Box<AST>, Vec<AST>, Vec<AST>),
    Loop(Vec<AST>),
    Break,

    ArrayNew(Box<AST>),                  
    Index(Box<AST>, Box<AST>),          
    AssignIndex(String, Box<AST>, Box<AST>),    
    ReactiveAssignIndex(String, Box<AST>, Box<AST>),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Addition,
    Multiplication,
    Division,
    Subtraction,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    NotEqual,
    Equal,
    Or,
    And,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Add,
    Mul,
    Div,
    Sub,
    Push(i32),
    Load(String),
    Store(String),
    StoreReactive(String, Box<AST>),
    StoreImmutable(String),
    Print,
    Println,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
    Or,
    And,
    Label(String),
    Jump(String),
    JumpIfZero(String),
    
    ArrayNew, // pops Integer(size), pushes Array(len=size, init 0)
    ArrayGet, // pops index(Integer) and array(Array), pushes element (evaluated to Integer if lazy)
    StoreIndex(String),               // pops value, pops index(Integer); mutates env[name] as array
    StoreIndexReactive(String, Box<AST>),// pops index(Integer); stores LazyInteger(ast) into env[name][index]
    PushImmutableContext,
    PopImmutableContext,
    ClearImmutableContext,
}