use sha2::{Digest, Sha256};
use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut hasher = Sha256::new();

    // Check if an argument is passed to the programme
    if args.len() == 2 {
        // A string has been passed to the programme, should be a filename
        let filename = &args[1];
        let mut f = File::open(filename).expect("no file found");
        let metadata = File::metadata(&f).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        hasher.update(buffer); // hash phrase
        let filehash = hasher.finalize();
        println!("The SHA-256 hash of file {} is :", filename);
        println!("{:#02x}", filehash);
    } else {
        println!("Error: this programme takes a file name as input, please provide one!")
    }

    Ok(())
}
