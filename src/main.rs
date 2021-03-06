use std::env;
use log::debug;

#[macro_use]
extern crate lazy_static;

mod commands;
mod fileutils;
mod instructions;
mod vim;

// *Super* Convenient function
fn s(s: &str) -> String {
    s.to_string()
}

/// Parses the binary absolute path and returns the binary name
fn parse_bin(bin_path: &str) -> &str {
    bin_path.split('/').last().unwrap()
}

#[derive(Debug, Clone)]
pub struct Flag {
    name: String,
    short: Option<String>,
    desc: String,
}

// Define all the flags
lazy_static! {
    static ref FLAGS: Vec<Flag> = vec![
        Flag {
            name: s("--help"),
            short: Some(s("-h")),
            desc: s("Prints the help string")
        },
        Flag {
            name: s("--list"),
            short: Some(s("-l")),
            desc: s("Show the list of all available themes"),
        },
        Flag {
            name: s("--current"),
            short: Some(s("-c")),
            desc: s("Show the current theme")
        },
        Flag {
            name: s("--switch"),
            short: Some(s("-s")),
            desc: s("Switch the theme to a new one.")
        },
        Flag {
            name: s("--with-vim"),
            short: None,
            desc: s("Also tries to set the same colorscheme in vim")
        },
        Flag {
            name: s("--find"),
            short: Some(s("-f")),
            desc: s("Search for a theme.\n\nPARAMETER:\n\t<theme_name>")
        },
    ];
    static ref HELP_FLAG: &'static Flag = &FLAGS[0];
    static ref LIST_FLAG: &'static Flag = &FLAGS[1];
    static ref CURR_FLAG: &'static Flag = &FLAGS[2];
    static ref SWITCH_FLAG: &'static Flag = &FLAGS[3];
    static ref WITH_VIM_OPTION: &'static Flag = &FLAGS[4];
    static ref SEARCH_FLAG: &'static Flag = &FLAGS[5];
}

pub fn parse_flags(args: &[String]) -> Vec<&Flag> {
    FLAGS.iter().filter(|f| args.contains(&f.name)).collect()
}

/// Parses the theme name from the arguments, ignoring all the options
pub fn parse_theme(args: &[String], flags: Vec<&Flag>) -> String {
    // Lets ignore the option flag name strings (--foo) - we already have those
    let fnames: Vec<&String> = flags.iter().map(|f| &f.name).collect();

    let nonflags = args
        .iter()
        .filter(|arg| !&fnames.contains(arg))
        .collect::<Vec<&String>>();

    if nonflags.len() <= 1 {
        s("")
    } else {
        s(nonflags.last().unwrap())
    }
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    debug!("args = {:?}", &args);
    let bin_name = parse_bin(&args[0]);
    let instruction = instructions::parse_instructions(&args, bin_name);
    match instruction {
        Err(err) => println!("ERROR: {}", err),
        Ok(ins) => {
            commands::handle(ins);
        }
    }
}
