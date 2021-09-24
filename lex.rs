use std::collections::HashMap;

fn lookup(token: &char) -> String{
	//define the hash map according to the token table
	let token_map: HashMap<char, &str> = [('+', "ADD_OP"), ('-', "SUB_OP"), ('*', "MUL_OP"), ('/', "DIV_OP"), (';', "SEMI"), ('(', "OP_PAR"), (')', "CLOSE_PAR")].iter().cloned().collect();
	//return the value that corresponds to the char key, errors out if invalid lexeme is passed in
	token_map.get(&token).expect("Value is not in token hash map").to_string()

}