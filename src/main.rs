use std::error::Error;
use std::io;
use std::process;
use std::ffi::OsString;
use std::fs::File;
use std::env;
use std::path::Path;

fn read_csv(filename:String) -> Result<(), Box<dyn Error>> {
    let mut file = match File::open(Path::new(filename.as_str())) {
        Ok(file) => {
            let mut rdr = csv::ReaderBuilder::new()
            .escape(Some(b'\\'))
            .flexible(true)
            .comment(Some(b'#'))
            .from_reader(file);
            let headers = rdr.headers()?.clone();
            for result in rdr.records() {
                let record = result?;
                println!("{:?}", record);
            }
        },
        Err(why) => panic!("error"),
    };
    Ok(())
}

// returns file name of the 
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected path to csv file")),
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
