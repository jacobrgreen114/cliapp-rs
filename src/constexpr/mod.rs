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

type Command<'a, R> = &'a (dyn Fn() -> R + Sync);

#[derive(Debug)]
enum CommandLineError {
    UnexpectedFlag(String),
    UnexpectedParameter(String),
    UnexpectedCommand(String),
    ExpectedValue(String),
    ExpectedSubcommand,
}

/// A command line executable.
trait Executable<R> {
    fn flags(&self) -> &[Flag];
    fn parameters(&self) -> &[Parameter];
    fn subcommands(&self) -> &[SubCommand<R>];
    fn command(&self) -> Option<Command<R>>;

    fn execute<T: AsRef<str>>(
        &self,
        mut args: impl Iterator<Item = T>,
    ) -> Result<R, CommandLineError> {
        let flags = self.flags();
        let params = self.parameters();
        let subcommands = self.subcommands();
        let command = self.command();

        while let Some(arg) = args.next() {
            // long name
            if arg.as_ref().is_long_name() {
                let arg_slice = &arg.as_ref()[2..];

                // parameter
                if let Some((name, value)) = split_parameter(arg_slice) {
                    if let Some(param) = params.find_by_long_name(name) {
                        param.set_value(format_parameter_value(value))
                    } else {
                        return Err(CommandLineError::UnexpectedParameter(
                            arg.as_ref().to_string(),
                        ));
                    }
                }
                // flag
                else {
                    if let Some(flag) = flags.find_by_long_name(arg_slice) {
                        flag.mark()
                    } else {
                        return Err(CommandLineError::UnexpectedFlag(arg.as_ref().to_string()));
                    }
                }
            }
            // short name
            else if arg.as_ref().is_short_name() {
                let arg_slice = &arg.as_ref()[1..];

                // flag
                if let Some(flag) = flags.find_by_short_name(arg_slice) {
                    flag.mark()
                } else if let Some(param) = params.find_by_short_name(arg_slice) {
                    if let Some(value) = args.next() {
                        param.set_value(format_parameter_value(value.as_ref()))
                    } else {
                        return Err(CommandLineError::ExpectedValue(arg.as_ref().to_string()));
                    }
                } else {
                    return Err(CommandLineError::UnexpectedFlag(arg.as_ref().to_string()));
                }
            }
            // command
            else {
                if let Some(command) = subcommands
                    .iter()
                    .find(|command| command.long_name() == arg.as_ref())
                {
                    return command.execute(args);
                } else {
                    return Err(CommandLineError::UnexpectedCommand(
                        arg.as_ref().to_string(),
                    ));
                }
            }
        }

        if let Some(command) = command {
            return Ok(command());
        } else {
            return Err(CommandLineError::ExpectedSubcommand);
        }
    }
}

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
}

trait FindExt<T> {
    fn find_by_long_name<'a>(&'a self, name: &str) -> Option<&'a T>;
    fn find_by_short_name<'a>(&'a self, name: &str) -> Option<&'a T>;
}

impl<T> FindExt<T> for [T]
where
    T: Argument,
{
    fn find_by_long_name<'a>(&'a self, name: &str) -> Option<&'a T> {
        self.iter().find(|arg| arg.long_name() == name)
    }

    fn find_by_short_name<'a>(&'a self, name: &str) -> Option<&'a T> {
        self.iter().find(|arg| arg.short_name() == name)
    }
}

trait IsNameExt {
    fn is_long_name(&self) -> bool;
    fn is_short_name(&self) -> bool;
}

impl IsNameExt for str {
    fn is_long_name(&self) -> bool {
        self.starts_with("--")
    }

    fn is_short_name(&self) -> bool {
        self.starts_with("-")
    }
}
