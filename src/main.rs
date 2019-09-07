use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::env;
use std::path::Path;


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
        let mut file = match File::open(Path::new(filename.as_str())) {
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
            let mut def = App{headers:headers, rows:data};
            return def
        },
        Err(why) => panic!("error"),
    };
    }

    pub fn sql_generate(&self) {
        let mut data = Vec::new();
        for head in self.headers.iter() {
            data.push(head);
        }

        let mut result = format!("INSERT INTO({}) VALUES()",  data.join(",").as_str());
       println!("{:?}", result)
    }
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
   let data = App::new().from_csv_file(file_name.into_string().unwrap()).sql_generate();
}
