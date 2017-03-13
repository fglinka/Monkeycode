/*
 *monkeycode - the reference implementation for the monkeycode encoding
 *Copyright © 2017 xeeF7
 *
 *Permission is hereby granted, free of charge, to any person obtaining
 *a copy of this software and associated documentation files (the "Software"),
 *to deal in the Software without restriction, including without limitation
 *the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *and/or sell copies of the Software, and to permit persons to whom the
 *Software is furnished to do so, subject to the following conditions:
 *
 *The above copyright notice and this permission notice shall be included
 *in all copies or substantial portions of the Software.
 *
 *THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 *EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 *OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 *IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 *DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 *TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
 *OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

mod encode;
mod decode;

#[macro_use]
extern crate lazy_static;
extern crate clap;

use clap::{App, Arg, SubCommand, ArgMatches};
use std::io;
use std::io::Write;

fn handle_decode(args : &ArgMatches) {
    if let Some(data) = args.value_of("data") {
        let vec = match decode::monkeycode_string_to_bytes(data) {
            None => {
                println!("Data is not monkeycode");
                return;
            },
            Some(vec) => vec
        };
        if args.is_present("no_utf8") {
                let mut out = io::stdout();
                out.write_all(vec.as_slice()).unwrap();
        }
        else if args.is_present("lossy_utf8") {
            println!("{}", String::from_utf8_lossy(vec.as_slice()));
        }
        else {
            match String::from_utf8(vec) {
                Ok(str) => println!("{}", str),
                Err(_) => println!("Data does not contain valid utf8 data.")
            }
        }
    }
}

fn handle_encode(args : &ArgMatches) {
    match args.value_of("data") {
        Some(data) => println!("{}", encode::string_to_monkeycode(data)),
        None => println!("Not yet implemented.")
    }
}

fn main() {
    let mut app = App::new("monkeycode")
        .version("0.1")
        .author("xeef7")
        .about("Encodes and decodes data using the monkeycode encoding")
        .subcommand(SubCommand::with_name("encode")
                    .about("Encode data to monkeycode.")
                    .arg(Arg::with_name("data")
                         .help("(Optional) data to be encoded. If not specified stdin will be used.")
                         .index(1)
                         .multiple(false)
                         .takes_value(true)))
        .subcommand(SubCommand::with_name("decode")
                    .about("Encode data to monkeycode.")
                    .arg(Arg::with_name("data")
                         .help("(Optional) data to be decoded. If not specified stdin will be used.")
                         .index(1)
                         .multiple(false)
                         .takes_value(true))
                    .arg(Arg::with_name("no_utf8")
                         .short("u")
                         .long("no_utf8")
                         .help("Dump raw data unmodified to stdout")
                         .takes_value(false))
                    .arg(Arg::with_name("lossy_utf8")
                         .short("l")
                         .long("lossy_utf8")
                         .help("Replace invalid characters with Questionmarks instead of terminating")
                         .takes_value(false)
                         .conflicts_with("no_utf8")));
    let matches = app.clone().get_matches();
    match matches.subcommand() {
        ("encode", Some(cmd)) => handle_encode(cmd),
        ("decode", Some(cmd)) => handle_decode(cmd),
        _ => app.print_help().unwrap()
    }
}
