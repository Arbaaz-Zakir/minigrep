use std::env;
use std::error::Error;
use std::fs;
use std::process;
use minigrep::{search, search_case_insensitive};
fn main() {
    //let _args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let input = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", input.query);

    // read file
    println!("In file {}", input.file_path);

    if let Err(e) = run(input) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
impl Config {
    fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {

        args.next(); 

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a valid file path"),
        };
        
        // let ignore_case = if !args.next().is_none() {
        //     if args[3] == "-ci" {
        //         true
        //     } else if args[3] == "-cs" {
        //         false
        //     } else {
        //         panic!("unrecognised command {}", args[3]);
        //     }
        // } else {
        //     env::var("IGNORE_CASE").is_ok()
        // };
        let ignore_case = match args.next().as_deref() {
            Some("-ci") => true,
            Some("-cs") => false,
            Some(_arg) => return Err("Unrecognised command: {arg}"),
            None => env::var("IGNORE_CASE").is_ok(),
            
        };


        Ok(Config {query, file_path, ignore_case})
    }    

}


fn run(config: Config) -> Result<(), Box<dyn Error>>{

    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else{
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

