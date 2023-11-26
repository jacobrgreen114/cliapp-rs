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
/// # Generic Parameters
/// [`R`] - The return type of the application.
///
/// # Example
/// ```rust
/// use cliutil::constexpr as cli;
///
/// static EXAMPLE_APPLICATION: cli::Application =
///     cli::Application::build()
///         .with_name("Example App")
///         .with_description("An example command line application")
///         .with_command(&app_main)
///         .build();  
///
/// fn main() {
///     match EXAMPLE_APPLICATION.run() {
///         Ok(ret) => {},  // handle return value
///         Err(err) => {}, // handle command line error
///     }
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
    command: Option<Callback<'a, R>>,
    help: bool,
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
            help: true,
        }
    }

    /// Parses the command line arguments from [std::env::args()](std::env::args) and dispatched to the appropriate command.
    pub fn run(&self) -> Result<R, CommandLineError> {
        let mut args = env::args();
        let _binary = args
            .next()
            .expect("Expected path to binary as first argument");
        self.execute(args)
    }

    /// Parses the provided command line arguments and dispatched to the appropriate command.
    ///
    /// Note: this function does not skip the first argument (the binary path) that [run()](Self::run) does.
    pub fn execute<T: AsRef<str>>(
        &self,
        args: impl Iterator<Item = T>,
    ) -> Result<R, CommandLineError> {
        Executable::execute(self, args)
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
    command: Option<Callback<'a, R>>,
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

    pub const fn with_parameters(mut self, params: &'a [Parameter<'a>]) -> Self {
        self.params = Some(params);
        self
    }

    pub const fn with_subcommands(mut self, subcommands: &'a [SubCommand<'a, R>]) -> Self {
        self.subcommands = Some(subcommands);
        self
    }

    pub const fn with_command(mut self, command: Callback<'a, R>) -> Self {
        self.command = Some(command);
        self
    }

    pub const fn with_help(mut self, enabled: bool) -> Self {
        self.help = enabled;
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

impl<R> Command<R> for Application<'_, R> {
    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        self.description
    }

    fn flags(&self) -> &[Flag] {
        self.flags
    }

    fn parameters(&self) -> &[Parameter] {
        self.params
    }

    fn subcommands(&self) -> &[SubCommand<R>] {
        self.subcommands
    }

    fn command(&self) -> Option<Callback<R>> {
        self.command
    }

    fn help_enabled(&self) -> bool {
        self.help
    }
}
