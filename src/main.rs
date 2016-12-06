extern crate clap;
extern crate textile;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .version(env!("CARGO_PKG_VERSION"))
                    .version_short("v")
                    .about("Renders Textile markup language into HTML")
                    .arg(Arg::with_name("INPUT")
                            .help("Input file")
                            .required(true)
                            .index(1))
                    .arg(Arg::with_name("OUTPUT")
                            .help("Output file")
                            .index(2))
                    .arg(Arg::with_name("parse")
                            .long("parse")
                            .help("Prints parse tree"))
                    .get_matches();

    let ref input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT");
    let parse = matches.is_present("parse");

    if let Some(ref output_file) = output {
        let mut f = File::create(&Path::new(output_file)).expect("Unable to create file");
        f.write_all(textile::render(Path::new(input)).as_bytes()).expect("Unable to write data");
    }
    if parse {
        println!("{:#?}", textile::parser::parse(Path::new(input)));
    }
}
