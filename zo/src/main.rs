use std::env;
use std::process;
use std::fs;

fn main() {
    //println!("Welcome to the Zo Programming Language!");
    let args: Vec<String> = env::args().collect();
    let args_length: usize = args.len();

    if args_length > 2 {
        println!("Usage: zo [script_name].zo");
        process::exit(64);
    } else if args_length == 2 {
        // Run the file
        run_zo_source(&args[1]);
    } else {
        // Run the prompt
    }

}

fn run_zo_source(path: &str) {
    eprintln!("Running zo source file at: {:?}", path);
    
    let mut error_message = String::new();

    error_message.push_str("[ZO] ERROR: Could not read file at path ");
    error_message.push_str(path);

    let bytes: String = fs::read_to_string(path).expect(&error_message);

    match run_zo(bytes.as_str()) {
        Ok(_) => {},
        Err(msg) => {
            log_error(&msg, "");
        }
    }
}

fn run_zo(bytes: &str) -> Result<(), ZoError> {
    //let scanner: Scanner = Scanner::new(bytes);
    //let tokens: Vec<Token> = scanner.scan_tokens();

    //for token : tokens.iter() {
    //    println!(token);
    //}

    println!("{:?}", bytes);
    
    Ok(())
}


// ---
// ZoError Structure
// ---
#[derive(Debug)]
struct ZoError {
    line: usize,
    message: String,
}

// ---
// Error Handling
// ---

fn error(ln: usize, msg: &str) -> ZoError {
    let error = ZoError { line: ln, message: String::from(msg) };
    
    log_error(&error, "");

    return error;
}

fn log_error(error: &ZoError, location: &str) {
    eprintln!("[Ln {} {}] ERROR: {}", error.line.to_string(), location, error.message);
}

