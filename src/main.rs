use std::env;

#[macro_use]
extern crate lazy_static;

/// Parses the binary absolute path and returns the binary name
fn parse_bin_name(bin_path: &str) -> &str {
    bin_path.split("/").last().unwrap()
}

#[derive(Clone)]
struct Flag {
    name: String,
    desc: String,
}

// Define all the flags
lazy_static! {
    static ref FLAGS: Vec<Flag> = vec![
        Flag {
            name: "help".to_string(),
            desc: "Prints the help string".to_string(),
        },
        Flag {
            name: "with-vim".to_string(),
            desc: "Also tries to set the same colorscheme in vim".to_string(),
        },
    ];
}

/// Generate the usage string for the tool
fn gen_usage(bin_name: &str) -> String {
    let topline = format!("{} <theme> [options]\n", bin_name);

    let mut secondline = format!("\nOPTIONS:\n");
    for flag in FLAGS.to_vec() {
        secondline.push_str(format!("\t--{}: \t {}\n", flag.name, flag.desc).as_str());
    }

    format!("{}{}", topline, secondline)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let bin_name = parse_bin_name(&args[0]);
    if args.len() == 1 || (&args[1] == "--help" || &args[1] == "-h") {
        let usage = gen_usage(&bin_name);
        println!("Usage:\n\t{}", usage);
    } else {
    }
}
