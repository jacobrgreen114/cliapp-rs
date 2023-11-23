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

use super::*;

/// A command line subcommand.
///
/// # Example
/// ```bash
/// $ ./myapp {subcommand} --{flag} --{parameter}={value}
/// ```
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

    pub const fn long_name(&self) -> &str {
        self.long_name
    }

    pub const fn description(&self) -> &str {
        self.description
    }

    pub const fn flags(&self) -> &[Flag] {
        self.flags
    }

    pub const fn parameters(&self) -> &[Parameter] {
        self.params
    }

    pub const fn subcommands(&self) -> &[SubCommand<R>] {
        self.subcommands
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

    pub const fn with_parameters(mut self, params: &'a [Parameter<'a>]) -> Self {
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

    fn parameters(&self) -> &[Parameter] {
        self.params
    }

    fn subcommands(&self) -> &[SubCommand<R>] {
        self.subcommands
    }

    fn command(&self) -> Option<Command<R>> {
        self.command
    }
}
