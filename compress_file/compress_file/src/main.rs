use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{copy, BufReader, Error, ErrorKind};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            "Usage: `source` `target`",
        )));
    }

    let source_path = &args[1];
    let target_path = &args[2];

    let mut input = BufReader::new(File::open(source_path)?);
    let output = File::create(target_path)?;

    let mut encoder = GzEncoder::new(output, Compression::default());
    let start_time = Instant::now();

    copy(&mut input, &mut encoder)?;

    let output_file = encoder.finish()?;

    println!(
        "Source len: {:?}",
        input.get_ref().metadata()?.len()
    );
    println!("Target len: {:?}", output_file.metadata()?.len());
    println!("Elapsed time: {:?}", start_time.elapsed());

    Ok(())
}
