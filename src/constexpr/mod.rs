use std::cell::UnsafeCell;
use std::env;
use std::env::Args;

type Command<'a, R> = &'a (dyn Fn() -> R + Sync);

/**

*/
#[derive(Debug)]
pub struct FlagValue {
    value: UnsafeCell<bool>,
}

impl FlagValue {
    pub const fn new() -> Self {
        Self {
            value: UnsafeCell::new(false),
        }
    }

    pub fn value(&self) -> bool {
        unsafe { *self.value.get() }
    }

    fn mark(&self) {
        unsafe { *self.value.get() = true }
    }
}

unsafe impl Sync for FlagValue {}

#[derive(Debug)]
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

// #[derive(Debug)]
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

// #[derive(Debug)]
pub struct SubCommand<'a, R> {
    //short_name: &'a str,
    long_name: &'a str,
    description: &'a str,
    flags: &'a [Flag<'a>],
    params: &'a [Parameter<'a>],
    subcommands: &'a [SubCommand<'a, R>],
    command: Option<Command<'a, R>>,
}

impl<'a, R> SubCommand<'a, R> {
    pub const fn new(
        long_name: &'a str,
        description: &'a str,
        flags: &'a [Flag],
        params: &'a [Parameter],
        subcommands: &'a [SubCommand<'a, R>],
        command: Option<Command<'a, R>>,
    ) -> Self {
        Self {
            long_name,
            description,
            flags,
            params,
            subcommands,
            command,
        }
    }

    fn run(&self, args: Args) -> R {
        todo!()
    }
}

/**
    Application
*/
pub struct Application<'a, R> {
    name: &'a str,
    description: &'a str,
    flags: &'a [Flag<'a>],
    params: &'a [Parameter<'a>],
    subcommands: &'a [SubCommand<'a, R>],
    command: Option<Command<'a, R>>,
}

impl<'a, R> Application<'a, R> {
    pub const fn new(
        name: &'a str,
        description: &'a str,
        flags: &'a [Flag],
        params: &'a [Parameter],
        subcommands: &'a [SubCommand<'a, R>],
        command: Option<Command<'a, R>>,
    ) -> Self {
        Self {
            name,
            description,
            flags,
            params,
            subcommands,
            command,
        }
    }

    pub fn run(&self) -> R {
        assert!(
            !(self.command.is_none() && self.subcommands.is_empty()),
            "Application is required to have a default command or at least one subcommand."
        );

        let mut args = env::args();
        if let None = args.next() {
            panic!("Expected first argument to be program.")
        }

        while let Some(arg) = args.next() {
            if arg.starts_with("--") {
                let arg_slice = &arg[2..];

                if let Some(equals_pos) = arg_slice.find('=') {
                    let param_name = &arg_slice[0..equals_pos];
                    let param_value = &arg_slice[equals_pos + 1..];

                    if let Some(param) = self
                        .params
                        .iter()
                        .find(|param| param.long_name == param_name)
                    {
                        param.set_value(param_value)
                    } else {
                        panic!("Unexpected parameter: \"{arg}\"")
                    }
                } else {
                    if let Some(flag) = self.flags.iter().find(|flag| flag.long_name == arg_slice) {
                        flag.mark()
                    } else {
                        panic!("Unexpected flag: \"{arg}\"")
                    }
                }
            } else if arg.starts_with("-") {
                let arg_slice = &arg[1..];

                if let Some(flag) = self.flags.iter().find(|flag| flag.short_name == arg_slice) {
                    flag.mark()
                } else {
                    panic!("Unexpected flag: \"{arg}\"")
                }
            } else {
                if let Some(command) = self
                    .subcommands
                    .iter()
                    .find(|command| command.long_name == arg)
                {
                    return command.run(args);
                } else {
                    panic!("Unexpected command: \"{arg}\"")
                }
            }
        }

        if let Some(command) = self.command {
            return command();
        } else {
            panic!("");
        }
    }
}
