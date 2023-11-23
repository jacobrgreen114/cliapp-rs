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

use std::env;

/// The root of a console application.
///
/// # Example
/// Application with a default command:
///
/// ```rust
/// use cliutil::constexpr as cli;
///
/// static EXAMPLE_APPLICATION: cli::Application =
///     cli::Application::build()
///         .with_name("Example Cli App")
///         .with_description("An example CLI application")
///         .with_command(&app_main)
///         .build();  
///
/// fn main() {
///     EXAMPLE_APPLICATION.run()
/// }
///  
/// fn app_main() {
///     println!("Hello, world!");
/// }
/// ```
pub struct Application<'a, R = ()> {
    name: &'a str,
    description: &'a str,
    flags: &'a [Flag<'a>],
    params: &'a [Parameter<'a>],
    subcommands: &'a [SubCommand<'a, R>],
    command: Option<Command<'a, R>>,
    // help: bool,
    // version: bool,
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
            //help: true,
        }
    }

    pub fn run(&self) -> R {
        let mut args = env::args();
        let _binary = args.next();
        self.execute(args)
    }

    pub fn execute<T: AsRef<str>>(&self, args: impl Iterator<Item = T>) -> R {
        match Executable::execute(self, args) {
            Ok(result) => result,
            Err(error) => {
                panic!("Error: {:?}", error);
            }
        }
    }

    pub const fn name(&self) -> &str {
        self.name
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

pub struct ApplicationBuilder<'a, R> {
    name: Option<&'a str>,
    description: Option<&'a str>,
    flags: Option<&'a [Flag<'a>]>,
    params: Option<&'a [Parameter<'a>]>,
    subcommands: Option<&'a [SubCommand<'a, R>]>,
    command: Option<Command<'a, R>>,
    // help: bool,
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

    // pub const fn with_help(mut self, help: bool) -> Self {
    //     self.help = help;
    //     self
    // }

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
            // help: self.help,
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
