use std::collections::HashMap;
use std::iter::Enumerate;
use std::process;

fn lookup(token: &char) -> String{
 
    //define the hash map according to the token table
    let token_map: HashMap<char, &str> = [('=', "ASSIGN"), ('+', "ADD_OP"), ('-', "SUB_OP"), ('*', "MUL_OP"), ('/', "DIV_OP"), (';', "SEMI"), ('(', "OP_PAR"), (')', "CL_PAR")].iter().cloned().collect();
    //return the value that corresponds to the char key, errors out if invalid lexeme is passed in
    token_map.get(&token).expect("Value is not in token hash map").to_string()

}

fn expr(lexemes: &Vec<String>, tokens: &Vec<String>){

    let mut iterator = tokens.iter().enumerate();

    println!("Enter <expr>");

    for (i, item) in &mut iterator{

        if(item == "ADD_OP" || item == "SUB_OP"){

            println!("Lexeme is: {} Token is: {}", lexemes[i], item);
            expr(&lexemes[(i+1)..].to_vec(), &tokens[(i+1)..].to_vec());
            println!("Exit <expr>");

        } else{

            term(&lexemes[i..].to_vec(), &tokens[i..].to_vec());

        }

    }

}

fn term(lexemes: &Vec<String>, tokens: &Vec<String>){

    let mut iterator = tokens.iter().enumerate();

    println!("Enter <term>");

    for (i, item) in &mut iterator{

        if(item == "MUL_OP" || item == "DIV_OP"){

            println!("Lexeme is: {} Token is: {}", lexemes[i], item);
            expr(&lexemes[(i+1)..].to_vec(), &tokens[(i+1)..].to_vec());
            println!("Exit <term>");

        } else{

            factor(&lexemes[i..].to_vec(), &tokens[i..].to_vec());

        }

    }
    println!("Exit <term>");

}

fn factor(lexemes: &Vec<String>, tokens: &Vec<String>){

    let mut iterator = tokens.iter().enumerate();

    println!("Enter <factor>");

    for (i, item) in &mut iterator{

        if(item == "ITY" || item == "INT_LIT"){

            println!("Lexeme is: {} Token is: {}", lexemes[i], item);
            expr(&lexemes[(i+1)..].to_vec(), &tokens[(i+1)..].to_vec());
            println!("Exit <factor>");

        } else if(item == "OP_PAR"){

            println!("Lexeme is: {} Token is: {}", lexemes[i], item);
            expr(&lexemes[(i+1)..].to_vec(), &tokens[(i+1)..].to_vec());


        } else if(item == "CL_PAR"){

             println!("Lexeme is: {} Token is: {}", lexemes[i], item);
             expr(&lexemes[(i+1)..].to_vec(), &tokens[(i+1)..].to_vec());
        }else if(item == "EOF"){

            process::exit(0);

        }



        // {

        //     expr(&lexemes[i..].to_vec(), &tokens[i..].to_vec());

        // }

    }

    println!("Exit <factor>");



}

fn lex(input_str: &str) -> (Vec<String>, Vec<String>) {
    
    let mut chars = input_str.chars().peekable();

    let mut lexemes = Vec::<String>::new();
    let mut tokens = Vec::<String>::new();
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
            tokens.push("ITY".to_string());
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
            tokens.push("INT_LIT".to_string());
            in_progress_lexeme = String::new();
        } else {
            lexemes.push(c.to_string());
            tokens.push(lookup(&c));
            chars.next();
        }
    }
    lexemes.push("EOF".to_string());
    tokens.push("EOF".to_string());
    (lexemes, tokens)
}

fn main(){

    let input_str = String::from("(sum+47)/total");
    let mut lexemes = lex(&input_str).0;
    let mut tokens = lex(&input_str).1;
    expr(&lexemes, &tokens);

}
