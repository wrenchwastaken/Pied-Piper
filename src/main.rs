extern crate flate2;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;  //as we'll be working with files as this is a compression algorithm
use std::io::copy;  //we need copy as when we compress the file and store the file it'll still be a copy of the original
use std::io::BufReader; //helps us to read the file
use std::time::Instant; //to show how much time it takes to copy and compress our files

fn main(){
    // this statement basically uses recursion to say that 3 arguments are needed for the program to run
    if args().len() != 3{
        println!("Usage: 'Source' 'Target'");
        return;
    }
    // this argument is argument for the source file, which has the file that we will be reading using the buffreader
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    // this argument is which decompresses the file to the target
    let output = File::create(args().nth(2).unwrap()).unwrap();
    // this is the line where the magic starts happening as this line is responsible for encoding our input
    let mut encoder = GzEncoder::new(output, Compression::default());
    //this line is to show how much time it takes to compress our file
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!("Source Length:{:?}", input.get_ref().metadata().unwrap().len());
    println!("Target Length:{:?}", output.metadata().unwrap().len());
    println!("Elapsed Time:{:?}",start.elapsed());
}
//cargo run filename.ext {name of the compressed file}