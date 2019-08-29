use std::error::Error;
use std::io;
use std::process;

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}


fn main() {
   if let Err(err) = read_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
