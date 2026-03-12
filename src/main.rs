extern crate flate2;

use flate2::Compression;
use flate2::write::GzEncoder;
use std::env::args; // taking cli arguments
use std::fs::File;
use std::io::BufReader;
use std::io::copy;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        println!("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unrape()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());

    // for checking time that take.
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!("Sourcxe len:{:?", input.get_ref().metadata.unwrap.len());
    println!("Target len:{:?}", output.metadata().unwrap().len());
    println!("Elapsed time: {:?}", start.elapsed());
}
