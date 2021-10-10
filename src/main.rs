use clap::{App, Arg};
use std::{
    env,
    fs::OpenOptions,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
    str,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const XOR_VAL: u8 = 0x5E;

fn main() {
    let mut clap_app = App::new("Obfuscator for streamDeckAudio")
        .version(VERSION)
        .about("Converts *.streamDeckAudio files to *.wav files and vice versa.\n(Every byte is XOR-ed with 0x5E)")
        .arg(
            Arg::with_name("input-file")
                .value_name("Input File")
                .short("i")
                .help("The file that should be converted.")
                .required(true)
                .index(1),
        );

    let mut help = Vec::new();
    clap_app.write_long_help(&mut help).unwrap();

    let help = str::from_utf8(&help).unwrap();
    let matches = clap_app.get_matches();

    let input_file: PathBuf = matches
        .value_of("input-file")
        .expect("Input file was missing.")
        .into();

    let result = transform_file(input_file);

    if let Err(ref error) = result {
        eprintln!("ERROR: {}", error);
        println!();
        println!("{}", help);

        std::process::exit(1);
    }
}

fn transform_file(input_file: PathBuf) -> Result<(), String> {
    let mut output_file = input_file.clone();

    swap_extension(&mut output_file)?;

    let input_handle = OpenOptions::new()
        .read(true)
        .open(input_file)
        .map_err(|_| "Can't read input file.".to_owned())?;

    let output_handle = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(output_file)
        .map_err(|_| "Can't write or create output file.".to_owned())?;

    let reader = BufReader::new(input_handle);
    let mut writer = BufWriter::new(output_handle);

    let mut iter = reader.bytes();

    let mut buffer_byte: u8 = 0;
    let single_byte_buffer = std::slice::from_mut(&mut buffer_byte);

    while let Some(Ok(x)) = iter.next() {
        single_byte_buffer[0] = x ^ XOR_VAL;
        writer.write_all(single_byte_buffer).unwrap();
    }

    Ok(())
}

fn swap_extension(file: &mut PathBuf) -> Result<(), String> {
    let extension = file
        .extension()
        .map(|x| x.to_str())
        .flatten()
        .ok_or_else(|| "Unsupported file extension format.".to_owned())
        .map(|x| x.to_ascii_lowercase())?;

    let new_extension = match extension.as_str() {
        "wav" => "streamDeckAudio",
        "streamdeckaudio" => "wav",
        _ => return Err("File extension not supported.".to_string()),
    };

    file.set_extension(new_extension);

    Ok(())
}
