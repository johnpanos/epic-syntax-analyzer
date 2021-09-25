# Epic Syntax Analyzer

## Getting Started

1. [Download and setup Rust](https://www.rust-lang.org/learn/get-started)
2. Clone the repository
```
$ git clone git@github.com:johnpanos/epic-syntax-analyzer.git
```
3. Compile
```
$ rustc ./main.rs
```
4. Run
```
$ ./main
```

## Token Table
| Token | Description |
|-------|-------------|
|ITY    |Variable     |
|ASSIGN |Assignment   |
|ADD_OP |Addition     |
|SUB_OP |Subtraction  |
|MUL_OP |Multiplication|
|DIV_OP |Division     |
|INT_LIT|Integer Literal|
|SEMI   |Semicolon    |
|OP_PAR |Open Paren   |
|CL_PAR |Close Paren  |

## `lookup(token: &char)`
`lookup()` takes a reference to a char as input and searches the pre-defined hash map, which is set up like the table above, and returns the string that corresponds to the character.
