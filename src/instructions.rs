use crate::s;
use crate::Flag;
use crate::FLAGS;
use itertools::Itertools;

#[derive(Debug)]
pub enum Command {
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

pub fn parse_flags(args: &[String]) -> Result<Vec<&Flag>, String> {
    let supported_flags: Vec<String> = FLAGS.iter().map(|flag| flag.name.clone()).collect();
    let mut has_invalid_flag = false;
    let mut bad_flag = "";
    args.iter()
        .filter(|arg| arg.starts_with("--"))
        .for_each(|arg| {
            if !supported_flags.contains(&arg) {
                has_invalid_flag = true;
                bad_flag = &arg;
                return;
            }
        });

    if !has_invalid_flag {
        Ok(FLAGS.iter().filter(|f| args.contains(&f.name)).collect())
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
    match parse_flags(&args) {
        Ok(flags) => {
            // let flags: Vec<&Flag> = parse_flags(&args);
            // Parse all flags and create a vector of all Command enum variants
            let instruction = Instruction {
                command: None,
                option: None,
            };

            let final_instruction: Result<Instruction, String> = flags
                .clone()
                .into_iter()
                .map(Ok)
                .fold_ok(instruction, |mut instruction_acc, parsed_flag| {
                    let Flag { name, .. } = parsed_flag;
                    println!("Considering: {}", &name);
                    match name.as_str() {
                        "--list" => instruction_acc.command = Some(Command::ListThemes),
                        "--current" => instruction_acc.command = Some(Command::CurrentTheme),
                        "--help" => {
                            instruction_acc.command = Some(Command::Help(bin_name.to_string()))
                        }
                        "--switch" => {
                            let theme = parse_theme(&args, flags.clone());
                            if !theme.is_empty() {
                                instruction_acc.command = Some(Command::SwitchTheme(theme))
                            } else {
                                println!("ERROR: No theme found!");
                            }
                        }
                        "--with-vim" => {
                            // TODO: Fix cases like Instruction { command: Some(ListThemes), option: Some(SwitchWithVim) }
                            instruction_acc.option = Some(CommandOption::SwitchWithVim);
                        }
                        _ => {
                            if instruction_acc.command.is_some() {
                                // Do nothing if we already found a command
                            } else {
                                println!("Should we do nothing or print an Error here?");
                                // No command found so far, keep it a Noop
                                instruction_acc.command = Some(Command::Noop)
                            }
                        }
                    };
                    instruction_acc
                });
            final_instruction
        }
        Err(e) => Err(e),
    }
}
