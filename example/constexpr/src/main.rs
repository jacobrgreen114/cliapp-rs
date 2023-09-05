extern crate cliapp;

use cliapp::constexpr::*;

static TEST_BOOL: FlagValue = FlagValue::new();

// command line flag (e.g. -t --test)
static EXAMPLE_FLAGS: &[Flag] = &[Flag::new(
    "t",
    "test",
    "Run the application in test mode",
    &TEST_BOOL,
)];

static EXAMPLE_PARAMETERS: &[Parameter] = &[];

static EXAMPLE_SUBCOMMANDS: &[SubCommand<()>] = &[SubCommand::new(
    "headless",
    "Runs teh program in headless mode",
    &[],
    &[],
    &[],
    Some(&headless_main),
)];

static EXAMPLE_APPLICATION: Application<()> = Application::new(
    "",
    "",
    EXAMPLE_FLAGS,
    EXAMPLE_PARAMETERS,
    EXAMPLE_SUBCOMMANDS,
    Some(&app_main),
);

fn main() {
    EXAMPLE_APPLICATION.run()
}

fn app_main() {
    println!("app_main()")
}

fn headless_main() {
    println!("headless_main")
}
