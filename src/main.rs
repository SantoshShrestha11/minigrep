use std::env;
use std::process;
// we are using the eprint to print the error as stander error to redirect it using the bash
// the type 2 std output which is error and can be redirected using 2>
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    // println!("Searching for {}",config.query);
    // println!("In file {}",config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    
}
