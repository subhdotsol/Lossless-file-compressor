// Cli entrypoint and argument handling
use std::fs::*;

fn main() {
    //opening a file
    let file = File::open("src/hello.txt");

    match file {
        Ok(f) => println!("File opened successfully: {:?}", f),
        Err(e) => println!("Error opening file: {:?}", e),
    }

    // reading the file as text
    let contents = read_to_string("src/hello.txt");
    match contents {
        Ok(ref c) => println!("File contents: {}", c),
        Err(ref e) => println!("Error reading file: {:?}", e),
    }

    // reading the file as raw bytes

    let bytes = read("src/hello.txt");
    match bytes {
        Ok(ref data) => {
            println!("Raw bytes are : {:?}", bytes);
            println!("Total bytes : {:?}", data.len());
        }
        Err(e) => {
            println!("Error : {}", e);
        }
    }
}
