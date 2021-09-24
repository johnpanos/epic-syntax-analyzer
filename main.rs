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
