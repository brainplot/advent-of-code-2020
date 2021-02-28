use std::io;

use clap::clap_app;
use clap::crate_authors;
use clap::crate_description;
use clap::crate_version;

use lazy_static::lazy_static;

use crate::input::{file_into_string, stdin_into_string};

lazy_static! {
    static ref ARGS: clap::ArgMatches = {
        clap_app!("Advent of Code 2020" =>
            (version: crate_version!())
            (author: crate_authors!())
            (about: crate_description!())
            (@arg input: [FILE] "Sets the input file to use")
            (@arg output: -o --output +takes_value [FILE] "Sets the output file to use")
        )
        .get_matches()
    };
}

pub fn input() -> io::Result<String> {
    ARGS.value_of_os("input").map_or_else(
        stdin_into_string, // default if input file is not provided
        file_into_string,
    )
}
