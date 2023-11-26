/*
 * Copyright 2023 Jacob R. Green
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[cfg(test)]
mod tests;

mod flags;

use std::fmt::Display;
pub use flags::{Flag, FlagValue};

mod parameters;
pub use parameters::{Parameter, ParameterValue};

mod subcommand;
pub use subcommand::SubCommand;

mod application;
pub use application::Application;

pub mod builders {
    pub use super::flags::FlagBuilder;
    pub use super::parameters::ParameterBuilder;
    pub use super::application::ApplicationBuilder;
    pub use super::subcommand::SubCommandBuilder;
}

type Callback<'a, R> = &'a (dyn Fn() -> R + Sync);

const CONSOLE_WIOTH: usize = 80;
const NAME_WIDTH: usize = 20;
const DESCRIPTION_WIDTH: usize = CONSOLE_WIOTH - NAME_WIDTH;

#[derive(Debug)]
pub enum CommandLineError {
    UnknownArgument(String),
    UnexpectedParameter(String),
    UnknownCommand(String),
    ExpectedValue(String),
    ExpectedSubcommand,
}

impl Display for CommandLineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandLineError::UnknownArgument(arg) => {
                write!(f, "Unknown argument: {}", arg)
            }
            CommandLineError::UnexpectedParameter(param) => {
                write!(f, "Unexpected parameter: {}", param)
            }
            CommandLineError::UnknownCommand(cmd) => {
                write!(f, "Unknown command: {}", cmd)
            }
            CommandLineError::ExpectedValue(arg) => {
                write!(f, "Expected value for argument: {}", arg)
            }
            CommandLineError::ExpectedSubcommand => {
                write!(f, "Expected subcommand")
            }
        }
    }
}

/// A command line executable.
trait Command<R> {
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    fn flags(&self) -> &[Flag];
    fn parameters(&self) -> &[Parameter];
    fn subcommands(&self) -> &[SubCommand<R>];
    fn command(&self) -> Option<Callback<R>>;

    fn help_enabled(&self) -> bool;

    fn print_help(&self) {
        let name = self.name();
        let description = self.description();
        let flags = self.flags();
        let parameters = self.parameters();
        let subcommands = self.subcommands();

        println!("{}", name);
        println!("{}", description);
        println!();

        if !flags.is_empty() {
            println!("Flags:");
            for flag in flags {
                print_help_for_argument(flag)
            }
            println!();
        }

        if !parameters.is_empty() {
            println!("Parameters:");
            for param in parameters {
                print_help_for_argument(param)
            }
            println!();
        }

        if !subcommands.is_empty() {
            println!("Subcommands:");
            for subcommand in subcommands {
                print_help_for_subcommand(subcommand)
            }
            println!();
        }
    }
}

fn print_help_for_argument<A: Argument>(a: &A) {
    let short_name = a.short_name();
    let long_name = a.long_name();
    let description = a.description();
    let mut line_index = 0;

    print!("  ");
    line_index += 2;
    if !short_name.is_empty() {
        print!("-{}", short_name);
        line_index += short_name.len() + 1;
        if !long_name.is_empty() {
            print!(", ");
            line_index += 2;
        }
    }

    if !long_name.is_empty() {
        print!("--{}", long_name);
        line_index += long_name.len() + 2;
    }

    print!("{}", " ".repeat(NAME_WIDTH.saturating_sub(line_index)));

    let desc_len = DESCRIPTION_WIDTH.min(description.len());

    println!("{}", &description[..desc_len]);
}

fn print_help_for_subcommand<R>(subcommand: &SubCommand<R>) {
    print!("  {}", subcommand.long_name());
    print!(
        "{}",
        " ".repeat(NAME_WIDTH.saturating_sub(subcommand.long_name().len() + 2))
    );

    let desc_len = DESCRIPTION_WIDTH.min(subcommand.description().len());
    println!("{}", &subcommand.description()[..desc_len]);
}

trait Executable<R> {
    fn execute<T: AsRef<str>, It: Iterator<Item = T>>(
        &self,
        args: It,
    ) -> Result<R, CommandLineError>;
}

impl<R, Ty: Command<R>> Executable<R> for Ty {
    fn execute<T: AsRef<str>, It: Iterator<Item = T>>(
        &self,
        mut args: It,
    ) -> Result<R, CommandLineError> {
        let flags = self.flags();
        let params = self.parameters();
        let subcommands = self.subcommands();
        let command = self.command();

        while let Some(arg) = args.next() {
            let arg = arg.as_ref();

            // long name (--example)
            if arg.is_long_name() {
                let arg_slice = &arg[2..];

                // flag
                if let Some(flag) = flags.find_by_long_name(arg_slice) {
                    flag.mark()
                }
                // parameter
                else if let Some((name, value)) = split_parameter(arg_slice) {
                    if let Some(param) = params.find_by_long_name(name) {
                        param.set_value(format_parameter_value(value))
                    } else {
                        return Err(CommandLineError::UnexpectedParameter(arg.to_string()));
                    }
                }
                // unknown argument
                else {
                    if self.help_enabled() {
                        eprintln!("Unknown argument: {}", arg);
                    }
                    return Err(CommandLineError::UnknownArgument(arg.to_string()));
                }
            }
            // short name (-e)
            else if arg.is_short_name() {
                let arg_slice = &arg[1..];

                // flag
                if let Some(flag) = flags.find_by_short_name(arg_slice) {
                    flag.mark()
                }
                // parameter
                else if let Some(param) = params.find_by_short_name(arg_slice) {
                    if let Some(value) = args.next() {
                        let value = value.as_ref();
                        param.set_value(format_parameter_value(value))
                    } else {
                        return Err(CommandLineError::ExpectedValue(arg.to_string()));
                    }
                }
                // unknown argument
                else {
                    if self.help_enabled() {
                        eprintln!("Unknown argument: {}", arg);
                    }
                    return Err(CommandLineError::UnknownArgument(arg.to_string()));
                }
            }
            // command
            else {
                if self.help_enabled() && arg == "help" {
                    self.print_help();
                    std::process::exit(0)
                }

                return if let Some(command) = subcommands
                    .iter()
                    .find(|command| command.long_name() == arg)
                {
                    command.execute(args)
                } else {
                    if self.help_enabled() {
                        eprintln!("Unknown command: {}", arg);
                    }
                    Err(CommandLineError::UnknownCommand(arg.to_string()))
                };
            }
        }

        if let Some(command) = command {
            return Ok(command());
        } else {
            return Err(CommandLineError::ExpectedSubcommand);
        }
    }
}

#[inline(always)]
fn split_parameter(arg: &str) -> Option<(&str, &str)> {
    if let Some(equals_pos) = arg.find('=') {
        Some((&arg[0..equals_pos], &arg[equals_pos + 1..]))
    } else {
        None
    }
}

fn format_parameter_value(value: &str) -> String {
    value.to_string()
}

trait Argument {
    fn long_name(&self) -> &str;
    fn short_name(&self) -> &str;
    fn description(&self) -> &str;
}

trait FindExt<T> {
    fn find_by_long_name(&self, name: &str) -> Option<&T>;
    fn find_by_short_name(&self, name: &str) -> Option<&T>;
}

impl<T> FindExt<T> for [T]
where
    T: Argument,
{
    #[inline(always)]
    fn find_by_long_name(&self, name: &str) -> Option<&T> {
        self.iter().find(|arg| arg.long_name() == name)
    }

    #[inline(always)]
    fn find_by_short_name(&self, name: &str) -> Option<&T> {
        self.iter().find(|arg| arg.short_name() == name)
    }
}

trait IsNameExt {
    fn is_long_name(&self) -> bool;
    fn is_short_name(&self) -> bool;
}

impl IsNameExt for str {
    #[inline(always)]
    fn is_long_name(&self) -> bool {
        self.starts_with("--")
    }
    #[inline(always)]
    fn is_short_name(&self) -> bool {
        self.starts_with("-")
    }
}
