use crate::*;
use itertools::Itertools;
use log::{error, warn};

#[derive(Debug)]
pub enum Command {
    SearchTheme(String),
    ListThemes,
    CurrentTheme,
    SwitchTheme(String),
    Help(String),
    Noop,
}

#[derive(Debug)]
pub enum CommandOption {
    SwitchWithVim,
}

#[derive(Debug)]
pub struct Instruction {
    pub command: Option<Command>,
    pub option: Option<CommandOption>,
}

/// Parse Flags
/// Inspects all the flags that we have and checks if we have any unsupported flag
/// If so, we bail out early and print the Error.
/// Otherwise, we return all the Supported Flags that we find
pub fn parse_flags(args: &[String]) -> Result<Vec<&Flag>, String> {
    let mut supported_flags: Vec<String> = vec![];
    FLAGS.iter()
         .flat_map(|flag| {
            let mut v = vec![flag.name.clone()];
            if flag.short.is_some() {
                v.push(flag.short.as_ref().unwrap().clone());
            }
            v
         })
         .for_each(|f| supported_flags.push(f));

    // debug!("Supported Flags = {:?}", &supported_flags);

    let mut has_invalid_flag = false;
    let mut bad_flag = "";
    args.iter()
        .filter(|arg| arg.starts_with('-'))
        .for_each(|arg| {
            // debug!("Considering arg = {}", &arg);
            if !supported_flags.contains(arg) {
                has_invalid_flag = true;
                bad_flag = arg;
            }
        });

    if !has_invalid_flag {
        Ok(
            FLAGS.iter()
                 .filter(|f| args.contains(&f.name) ||
                    (f.short.is_some() && args.contains(f.short.as_ref().unwrap()))
                 ).collect()
        )
    } else {
        Err(format!("Invalid Flag: {}", bad_flag))
    }
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

/// parse the flags and return Instruction objects
/// We go through the arguments one by one and create an instruction object
/// containing the actual command as well as (for now) a single option.
/// once we have that Instruction, we return it.
/// In case we do not have a successful parse, we return an Error.
pub fn parse_instructions(args: &[String], bin_name: &str) -> Result<Instruction, String> {
    if args.len() == 1 {
        Ok(Instruction {
            command: Some(Command::Help(bin_name.to_string())),
            option: None,
        })
    } else {
        match parse_flags(args) {
            Ok(flags) => build_instruction(args, bin_name, flags),
            Err(e) => Err(e),
        }
    }
}

/// Checks if a given argument passed as a flag actually matches
/// our internal flag objects - either full name or the short
fn is_valid_flag(flag_arg: &str, flag_obj: &'static Flag) -> bool {
    flag_arg == flag_obj.name ||
    flag_arg == flag_obj.short.as_ref().unwrap()
}

/// Build an Instruction
/// This uses all the information we have about the Flags we have parsed
/// and looks at any additional things we need from the args (like values and options)
/// and builds out an Instruction
fn build_instruction(
    args: &[String],
    bin_name: &str,
    flags: Vec<&Flag>,
) -> Result<Instruction, String> {
    // let flags: Vec<&Flag> = parse_flags(&args);
    // Parse all flags and create a vector of all Command enum variants
    let instruction = Instruction {
        command: None,
        option: None,
    };

    // TODO: This whole thing breaks without `flags.clone()`. Can we avoid a clone here? Should we?
    // What is the _cost_ of doing a clone? How liberally should we use it? When should we
    // avoid it?
    // debug!("{:?}", &flags);
    let final_instruction: Result<Instruction, String> = flags.clone().into_iter().map(Ok).fold_ok(
        instruction,
        |mut instruction_acc, parsed_flag| {
            let Flag { name, .. } = parsed_flag;
            match name.as_str() {
                flag_name if is_valid_flag(flag_name, &LIST_FLAG) => {
                    instruction_acc.command = Some(Command::ListThemes)
                }
                flag_name if is_valid_flag(flag_name, &CURR_FLAG) => {
                    instruction_acc.command = Some(Command::CurrentTheme)
                }
                flag_name if is_valid_flag(flag_name, &HELP_FLAG) => {
                    instruction_acc.command = Some(Command::Help(bin_name.to_string()))
                }
                flag_name if is_valid_flag(flag_name, &SEARCH_FLAG) => {
                    let theme = parse_theme(args, flags.clone());
                    if !theme.is_empty() {
                        instruction_acc.command = Some(Command::SearchTheme(theme))
                    } else {
                        error!("Please provide a non-empty theme name!");
                    }
                }
                flag_name if is_valid_flag(flag_name, &SWITCH_FLAG) => {
                    let theme = parse_theme(args, flags.clone());
                    if !theme.is_empty() {
                        instruction_acc.command = Some(Command::SwitchTheme(theme))
                    } else {
                        error!("No theme found!");
                    }
                }
                flag_name if flag_name == WITH_VIM_OPTION.name => {
                    // Only match --with-vim when we have seen the switch command
                    match instruction_acc.command {
                        Some(Command::SwitchTheme(_)) => {
                            instruction_acc.option = Some(CommandOption::SwitchWithVim)
                        }
                        None => instruction_acc.option = Some(CommandOption::SwitchWithVim),
                        // TODO: Bail out early when this error is encountered.
                        Some(ref cmd) => {
                            warn!(
                                "--with-vim should not be used with {:?} command! Ignoring it",
                                &cmd
                            );
                        }
                    }
                }
                _ => {
                    if instruction_acc.command.is_some() {
                        // Do nothing if we already found a command
                    } else {
                        warn!("Should we do nothing or print an Error here?");
                        // No command found so far, keep it a Noop
                        instruction_acc.command = Some(Command::Noop)
                    }
                }
            };
            instruction_acc
        },
    );
    final_instruction
}
