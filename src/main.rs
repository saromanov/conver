use std::error::Error;
use std::io;
use std::process;
use std::ffi::OsString;
use std::fs::File;
use std::env;

fn read_csv(filename:String) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

// returns file name of the 
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    
    let file_name = get_first_arg().unwrap();
   if let Err(err) = read_csv(file_name.into_string().unwrap()) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
