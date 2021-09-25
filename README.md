# Epic Syntax Analyzer

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
