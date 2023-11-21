//
// Copyright 2023 Jacob R. Green
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::cell::Cell;
use std::env;
use std::fmt::Display;

type Command<'a, R> = &'a (dyn Fn() -> R + Sync);

pub struct FlagValue {
    value: Cell<bool>,
}

impl FlagValue {
    pub const fn new() -> Self {
        Self {
            value: Cell::new(false),
        }
    }

    pub fn value(&self) -> bool {
        unsafe { self.value.get() }
    }

    fn mark(&self) {
        unsafe { self.value.set(true) }
    }
}

unsafe impl Sync for FlagValue {}

pub struct Flag<'a> {
    short_name: &'a str,
    long_name: &'a str,
    description: &'a str,
    flag: &'a FlagValue,
}

impl<'a> Flag<'a> {
    pub const fn new(
        short_name: &'a str,
        long_name: &'a str,
        description: &'a str,
        flag: &'a FlagValue,
    ) -> Self {
        assert!(
            !(short_name.is_empty() && long_name.is_empty()),
            "Flag must have either a short name or long name."
        );
        Self {
            short_name,
            long_name,
            description,
            flag,
        }
    }

    fn mark(&self) {
        self.flag.mark();
    }
}

pub struct Parameter<'a> {
    short_name: &'a str,
    long_name: &'a str,
    description: &'a str,
}

impl<'a> Parameter<'a> {
    pub const fn new(short_name: &'a str, long_name: &'a str, description: &'a str) -> Self {
        Self {
            short_name,
            long_name,
            description,
        }
    }

    fn set_value(&self, value: &str) {}
}

trait Executable<R> {
    fn flags(&self) -> &[Flag];
    fn params(&self) -> &[Parameter];
    fn subcommands(&self) -> &[SubCommand<R>];
    fn command(&self) -> Option<Command<R>>;

    fn execute<T: AsRef<str> + Display>(&self, mut args: impl Iterator<Item = T>) -> R {
        let flags = self.flags();
        let params = self.params();
        let subcommands = self.subcommands();
        let command = self.command();

        let mut args = args.peekable();

        while let Some(arg) = args.next() {
            // long name
            if arg.as_ref().starts_with("--") {
                let arg_slice = &arg.as_ref()[2..];

                // parameter
                if let Some(equals_pos) = arg_slice.find('=') {
                    let param_name = &arg_slice[0..equals_pos];
                    let param_value = &arg_slice[equals_pos + 1..];

                    if let Some(param) = params.iter().find(|param| param.long_name == param_name) {
                        param.set_value(param_value)
                    } else {
                        panic!("Unexpected parameter: \"{}\"", arg)
                    }
                }
                // flag
                else {
                    if let Some(flag) = flags.iter().find(|flag| flag.long_name == arg_slice) {
                        flag.mark()
                    } else {
                        panic!("Unexpected flag: \"{}\"", arg)
                    }
                }
            }
            // short name
            else if arg.as_ref().starts_with("-") {
                let arg_slice = &arg.as_ref()[1..];

                // flag
                if let Some(flag) = flags.iter().find(|flag| flag.short_name == arg_slice) {
                    flag.mark()
                } else {
                    panic!("Unexpected flag: \"{}\"", arg)
                }
            }
            // command
            else {
                if let Some(command) = subcommands
                    .iter()
                    .find(|command| command.long_name == arg.as_ref())
                {
                    return command.execute(args);
                } else {
                    panic!("Unexpected command: \"{}\"", arg)
                }
            }
        }

        if let Some(command) = command {
            return command();
        } else {
            // todo - subcommand required
            panic!("");
        }
    }
}

pub struct SubCommand<'a, R = ()> {
    //short_name: &'a str,
    long_name: &'a str,
    description: &'a str,
    flags: &'a [Flag<'a>],
    params: &'a [Parameter<'a>],
    subcommands: &'a [SubCommand<'a, R>],
    command: Option<Command<'a, R>>,
}

impl<'a, R> SubCommand<'a, R> {
    pub const fn build() -> SubCommandBuilder<'a, R> {
        SubCommandBuilder {
            long_name: None,
            description: None,
            flags: None,
            params: None,
            subcommands: None,
            command: None,
        }
    }
}

pub struct SubCommandBuilder<'a, R> {
    //short_name: &'a str,
    long_name: Option<&'a str>,
    description: Option<&'a str>,
    flags: Option<&'a [Flag<'a>]>,
    params: Option<&'a [Parameter<'a>]>,
    subcommands: Option<&'a [SubCommand<'a, R>]>,
    command: Option<Command<'a, R>>,
}

impl<'a, R> SubCommandBuilder<'a, R> {
    pub const fn with_long_name(mut self, long_name: &'a str) -> Self {
        self.long_name = Some(long_name);
        self
    }

    pub const fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub const fn with_flags(mut self, flags: &'a [Flag<'a>]) -> Self {
        self.flags = Some(flags);
        self
    }

    pub const fn with_params(mut self, params: &'a [Parameter<'a>]) -> Self {
        self.params = Some(params);
        self
    }

    pub const fn with_subcommands(mut self, subcommands: &'a [SubCommand<'a, R>]) -> Self {
        self.subcommands = Some(subcommands);
        self
    }

    pub const fn with_command(mut self, command: Command<'a, R>) -> Self {
        self.command = Some(command);
        self
    }

    pub const fn build(self) -> SubCommand<'a, R> {
        let subcommand = SubCommand {
            long_name: match self.long_name {
                Some(long_name) => long_name,
                None => panic!("Subcommand must have a long name."),
            },
            description: match self.description {
                Some(description) => description,
                None => "",
            },
            flags: match self.flags {
                Some(flags) => flags,
                None => &[],
            },
            params: match self.params {
                Some(params) => params,
                None => &[],
            },
            subcommands: match self.subcommands {
                Some(subcommands) => subcommands,
                None => &[],
            },
            command: self.command,
        };

        assert!(
            subcommand.command.is_some() || !subcommand.subcommands.is_empty(),
            "Subcommand must have either a default command or at least one subcommand."
        );

        subcommand
    }
}

impl<R> Executable<R> for SubCommand<'_, R> {
    fn flags(&self) -> &[Flag] {
        self.flags
    }

    fn params(&self) -> &[Parameter] {
        self.params
    }

    fn subcommands(&self) -> &[SubCommand<R>] {
        self.subcommands
    }

    fn command(&self) -> Option<Command<R>> {
        self.command
    }
}

///
/// Application
///
pub struct Application<'a, R = ()> {
    name: &'a str,
    description: &'a str,
    flags: &'a [Flag<'a>],
    params: &'a [Parameter<'a>],
    subcommands: &'a [SubCommand<'a, R>],
    command: Option<Command<'a, R>>,
    help: bool,
}

impl<'a, R> Application<'a, R> {
    pub const fn build() -> ApplicationBuilder<'a, R> {
        ApplicationBuilder {
            name: None,
            description: None,
            flags: None,
            params: None,
            subcommands: None,
            command: None,
            help: true,
        }
    }

    pub fn run(&self) -> R {
        let mut args = env::args();
        let _binary = args.next();
        self.execute(args)
    }
}

pub struct ApplicationBuilder<'a, R> {
    name: Option<&'a str>,
    description: Option<&'a str>,
    flags: Option<&'a [Flag<'a>]>,
    params: Option<&'a [Parameter<'a>]>,
    subcommands: Option<&'a [SubCommand<'a, R>]>,
    command: Option<Command<'a, R>>,
    help: bool,
}

impl<'a, R> ApplicationBuilder<'a, R> {
    pub const fn with_name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub const fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub const fn with_flags(mut self, flags: &'a [Flag<'a>]) -> Self {
        self.flags = Some(flags);
        self
    }

    pub const fn with_params(mut self, params: &'a [Parameter<'a>]) -> Self {
        self.params = Some(params);
        self
    }

    pub const fn with_subcommands(mut self, subcommands: &'a [SubCommand<'a, R>]) -> Self {
        self.subcommands = Some(subcommands);
        self
    }

    pub const fn with_command(mut self, command: Command<'a, R>) -> Self {
        self.command = Some(command);
        self
    }

    pub const fn with_help(mut self, help: bool) -> Self {
        self.help = help;
        self
    }

    pub const fn build(self) -> Application<'a, R> {
        let app = Application {
            name: match self.name {
                Some(name) => name,
                None => "",
            },
            description: match self.description {
                Some(description) => description,
                None => "",
            },
            flags: match self.flags {
                Some(flags) => flags,
                None => &[],
            },
            params: match self.params {
                Some(params) => params,
                None => &[],
            },
            subcommands: match self.subcommands {
                Some(subcommands) => subcommands,
                None => &[],
            },
            command: self.command,
            help: self.help,
        };
        assert!(
            app.command.is_some() || !app.subcommands.is_empty(),
            "Application must have either a default command or at least one subcommand."
        );
        app
    }
}

impl<R> Executable<R> for Application<'_, R> {
    fn flags(&self) -> &[Flag] {
        self.flags
    }

    fn params(&self) -> &[Parameter] {
        self.params
    }

    fn subcommands(&self) -> &[SubCommand<R>] {
        self.subcommands
    }

    fn command(&self) -> Option<Command<R>> {
        self.command
    }
}
