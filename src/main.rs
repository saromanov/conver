use std::error::Error;
use std::io;
use std::process;
use std::ffi::OsString;
use std::fs::File;
use std::env;
use std::path::Path;


struct Definition {
    headers:csv::StringRecord,
    rows:Vec<csv::StringRecord>
}

pub struct App
{
}

impl App {
    pub fn new() -> Self {
        App {

        }
    }

    pub fn from_csv_file(filename:String) -> Self {
        let mut file = match File::open(Path::new(filename.as_str())) {
        Ok(file) => {
            let mut rdr = csv::ReaderBuilder::new()
            .escape(Some(b'\\'))
            .flexible(true)
            .comment(Some(b'#'))
            .from_reader(file);
            let headers = rdr.headers()?.clone();
            let mut data = Vec::new();
            for result in rdr.records() {
                data.push(result?)
            }
            let def = Definition{headers:headers, rows:data};
            Ok(def)
        },
        Err(why) => panic!("error"),
    };
    panic!("error")
    }
}

fn sql_generate(headers:String, rows:String) -> Result<(), String> {
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
   let data = App::new().read_from_csv(file_name.into_string().unwrap());
}
