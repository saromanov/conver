use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::env;
use std::path::Path;
use std::io::prelude::*;

pub struct App
{
    headers:csv::StringRecord,
    rows:Vec<csv::StringRecord>
}

impl App {
    pub fn new() -> Self {
        App {
            headers:csv::StringRecord::new(),
            rows:Vec::new()
        }
    }

    pub fn from_csv_file(self, filename:String) -> Self {
        match File::open(Path::new(filename.as_str())) {
        Ok(file) => {
            let mut rdr = csv::ReaderBuilder::new()
            .escape(Some(b'\\'))
            .flexible(true)
            .comment(Some(b'#'))
            .from_reader(file);
            let headers = rdr.headers().unwrap().clone();
            let mut data = Vec::new();
            for result in rdr.records() {
                data.push(result.unwrap())
            }
            return App{headers:headers, rows:data};
        },
        Err(_) => panic!("error"),
    };
    }

    pub fn sql_generate(&self, table_name:&str) {
        let data = get_str_from_string_record(self.headers.clone());
        let mut result = format!("INSERT INTO {} ({}) VALUES ", table_name, data.as_str());
        for v in self.rows.clone() {
            let d = format!("({}),", get_str_from_string_record(v));
            result += d.as_str();
        }
        result.truncate(result.len()- 1);
        result += ";";
        println!("{:?}", result)
    }

    pub fn save_to_file(&self, file_name:&str) -> App {
        let mut file = File::create(file_name).unwrap();
        file.write_all(b"Hello, world!").unwrap();
        App{headers:self.headers.clone(), rows:self.rows.clone()}
    }
}

pub fn get_str_from_string_record(inp:csv::StringRecord) -> String {
    let mut data = Vec::new();
    for head in inp.iter() {
        data.push(head);
    }
    data.join(",")
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
   App::new().from_csv_file(file_name.into_string().unwrap()).sql_generate(&"val");
}
