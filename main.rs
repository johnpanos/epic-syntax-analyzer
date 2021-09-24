use std::collections::HashMap;

fn lookup(token: &char) -> String{
    //define the hash map according to the token table
    let token_map: HashMap<char, &str> = [('=', "ASSIGN"), ('+', "ADD_OP"), ('-', "SUB_OP"), ('*', "MUL_OP"), ('/', "DIV_OP"), (';', "SEMI"), ('(', "OP_PAR"), (')', "CLOSE_PAR")].iter().cloned().collect();
    //return the value that corresponds to the char key, errors out if invalid lexeme is passed in
    token_map.get(&token).expect("Value is not in token hash map").to_string()

}

fn main() {
    let input_str = "(sum+47)/total";
    let mut chars = input_str.chars().peekable();

    let mut lexemes = Vec::<String>::new();
    let mut in_progress_lexeme = String::new();

    while let Some(c) = chars.peek() {
        if c.is_alphabetic() {
            while let Some(ch) = chars.peek() {
                if (ch.is_alphabetic()) {
                    in_progress_lexeme.push(*ch);
                } else {
                    break;
                }
                chars.next();
            }
            lexemes.push(in_progress_lexeme);
            in_progress_lexeme = String::new();
        } else if c.is_alphanumeric() {
            while let Some(ch) = chars.peek() {
                if (ch.is_alphanumeric()) {
                    in_progress_lexeme.push(*ch);
                } else {
                    break;
                }
                chars.next();
            }
            lexemes.push(in_progress_lexeme);
            in_progress_lexeme = String::new();
        } else {
            lexemes.push(c.to_string());
            chars.next();
        }
    }
    
    println!("{:?}", lexemes);
}
