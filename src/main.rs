use std::env;

#[macro_use]
extern crate lazy_static;

// *Super* Convenient function
fn s(s: &str) -> String {
    s.to_string()
}

/// Parses the binary absolute path and returns the binary name
fn parse_bin(bin_path: &str) -> &str {
    bin_path.split('/').last().unwrap()
}

#[derive(Debug, Clone)]
struct Flag {
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
            short: None,
            desc: s("Show the list of all available themes"),
        },
        Flag {
            name: s("--current"),
            short: None,
            desc: s("Show the current theme")
        },
        Flag {
            name: s("--with-vim"),
            short: None,
            desc: s("Also tries to set the same colorscheme in vim")
        },
        Flag {
            name: s("--switch"),
            short: None,
            desc: s("Switch the theme to a new one.\nSWITCH PARAMETER:\n\t<theme_name>")
        },
    ];
    static ref HELP_FLAG: &'static Flag = &FLAGS[0];
    static ref LIST_FLAG: &'static Flag = &FLAGS[1];
    static ref CURR_FLAG: &'static Flag = &FLAGS[2];
    static ref WITH_VIM_OPTION: &'static Flag = &FLAGS[3];
    static ref SWITCH_FLAG: &'static Flag = &FLAGS[4];
}

/// Generate the usage string for the tool
fn gen_usage(bin_name: &str) -> String {
    let topline = format!("{} [options] <parameter>\n", bin_name);

    let mut secondline = s("\nOPTIONS:\n");
    for flag in FLAGS.to_vec() {
        secondline.push_str(format!("\t{}: \t {}\n", flag.name, flag.desc).as_str());
    }
    format!("{}{}", topline, secondline)
}

fn parse_flags(args: &[String]) -> Vec<&Flag> {
    FLAGS.iter().filter(|f| args.contains(&f.name)).collect()
}

/// Parses the theme name from the arguments, ignoring all the options
fn parse_theme(args: &[String], flags: Vec<&Flag>) -> String {
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

#[derive(Debug)]
enum Command {
    ListThemes,
    CurrentTheme,
    SwitchTheme(String),
    Noop,
}

// enum ExtraOption {
//     WithVim,
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let bin_name = parse_bin(&args[0]);
    if args.len() == 1
        || (args[1] == HELP_FLAG.name || &args[1] == HELP_FLAG.short.as_ref().unwrap())
    {
        let usage = gen_usage(bin_name);
        println!("Usage:\n\t{}", usage);
    } else {
        let flags = parse_flags(&args);
        // Parse all flags and create a vector of all Command enum variants

        let cmds: Result<Vec<Command>, String> = flags
            .clone()
            .into_iter()
            .map(|parsed_flag| {
                let Flag { name, .. } = parsed_flag;
                if name == &LIST_FLAG.name {
                    Ok(Command::ListThemes)
                } else if name == &SWITCH_FLAG.name {
                    let theme = parse_theme(&args, flags.clone());
                    if theme.is_empty() {
                        Err(s("No Theme Provided"))
                    } else {
                        Ok(Command::SwitchTheme(theme))
                    }
                } else if name == &CURR_FLAG.name {
                    Ok(Command::CurrentTheme)
                } else {
                    Ok(Command::Noop)
                }
            })
            .collect::<Result<Vec<Command>, String>>();

        // Now that we have a bunch of commands, we can execute them,
        // or return with an error when that is suitable
        match cmds {
            Err(err) => {
                println!("Error = {:?}", err);
            }
            Ok(commands) => {
                println!("commands = {:?}", commands);
            }
        }
    }
}
