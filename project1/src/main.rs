use std::fs::File;
use std::io::{BufRead, BufReader};

fn test(s:String) -> String {
    let mut x = 1;
    let mut y = x;
    
}
fn main() -> Result<(), std::io::Error> {

    let mut nome = String;
    let resultado: String = test(&nome);



    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}