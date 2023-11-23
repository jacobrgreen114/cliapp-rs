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

extern crate cliutil;

use cliutil::constexpr as cli;

static TEST_BOOL: cli::FlagValue = cli::FlagValue::new();

// command line flag (e.g. -t --test)
static EXAMPLE_FLAGS: &[cli::Flag] = &[cli::Flag::build()
    .with_short_name("t")
    .with_long_name("test")
    .with_description("A test flag")
    .with_flag(&TEST_BOOL)
    .build()];

static TEST_PARAM: cli::ParameterValue = cli::ParameterValue::new();

static EXAMPLE_PARAMETERS: &[cli::Parameter] = &[
    cli::Parameter::build()
        .with_short_name("p")
        .with_long_name("param")
        .with_description("A test parameter")
        .with_parameter(&TEST_PARAM)
        .build(),
];

static EXAMPLE_SUBCOMMANDS: &[cli::SubCommand] = &[cli::SubCommand::build()
    .with_long_name("headless")
    .with_description("Runs the program in headless mode")
    .with_command(&headless_main)
    .build()];

static EXAMPLE_APPLICATION: cli::Application = cli::Application::build()
    .with_name("Example Cli App")
    .with_description("An example CLI application")
    .with_flags(EXAMPLE_FLAGS)
    .with_parameters(EXAMPLE_PARAMETERS)
    .with_subcommands(EXAMPLE_SUBCOMMANDS)
    .with_command(&app_main)
    .build();

fn main() {
    EXAMPLE_APPLICATION.run()
}

fn app_main() {
    println!("app_main()")
}

fn headless_main() {
    println!("headless_main()")
}
