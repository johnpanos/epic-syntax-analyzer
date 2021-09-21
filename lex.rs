use std::collections::HashMap;

fn lookup(token: &char) -> String{

	let token_map: HashMap<char, &str> = [('+', "ADD_OP"), ('-', "SUB_OP"), ('*', "MUL_OP"), ('/', "DIV_OP"), (';', "SEMI"), ('(', "OP_PAR"), (')', "CLOSE_PAR")].iter().cloned().collect();
	token_map.get(&token).expect("Value is not in token hash map").to_string()

}

fn main(){

	let token = lookup(&' ');
	println!("{}", token);

}