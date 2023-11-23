use super::*;

#[test]
fn flag_parsing_1() {
    let f: FlagValue = FlagValue::new();
    let g: FlagValue = FlagValue::new();

    let flags: &[Flag] = &[
        Flag::build()
            .with_short_name("f")
            .with_long_name("flag")
            .with_description("A flag")
            .with_flag(&f)
            .build(),
        Flag::build()
            .with_short_name("g")
            .with_long_name("gflag")
            .with_description("A gflag")
            .with_flag(&g)
            .build(),
    ];

    let app: Application = Application::build()
        .with_flags(flags)
        .with_command(&default_command)
        .build();

    let args: [&str; 0] = [];

    app.execute(args.iter());

    assert!(!f.value() && !g.value());
}

#[test]
fn flag_parsing_2() {
    let f: FlagValue = FlagValue::new();
    let g: FlagValue = FlagValue::new();

    let flags: &[Flag] = &[
        Flag::build()
            .with_short_name("f")
            .with_long_name("flag")
            .with_description("A flag")
            .with_flag(&f)
            .build(),
        Flag::build()
            .with_short_name("g")
            .with_long_name("gflag")
            .with_description("A gflag")
            .with_flag(&g)
            .build(),
    ];

    let app: Application = Application::build()
        .with_flags(flags)
        .with_command(&default_command)
        .build();

    let args: [&str; 1] = ["-f"];

    app.execute(args.iter());

    assert!(f.value() && !g.value());
}

#[test]
fn flag_parsing_3() {
    let f: FlagValue = FlagValue::new();
    let g: FlagValue = FlagValue::new();

    let flags: &[Flag] = &[
        Flag::build()
            .with_short_name("f")
            .with_long_name("flag")
            .with_description("A flag")
            .with_flag(&f)
            .build(),
        Flag::build()
            .with_short_name("g")
            .with_long_name("gflag")
            .with_description("A gflag")
            .with_flag(&g)
            .build(),
    ];

    let app: Application = Application::build()
        .with_flags(flags)
        .with_command(&default_command)
        .build();

    let args: [&str; 1] = ["-g"];

    app.execute(args.iter());

    assert!(!f.value() && g.value());
}

#[test]
fn flag_parsing_4() {
    let f: FlagValue = FlagValue::new();
    let g: FlagValue = FlagValue::new();

    let flags: &[Flag] = &[
        Flag::build()
            .with_short_name("f")
            .with_long_name("flag")
            .with_description("A flag")
            .with_flag(&f)
            .build(),
        Flag::build()
            .with_short_name("g")
            .with_long_name("gflag")
            .with_description("A gflag")
            .with_flag(&g)
            .build(),
    ];

    let app: Application = Application::build()
        .with_flags(flags)
        .with_command(&default_command)
        .build();

    let args: [&str; 2] = ["-f", "-g"];

    app.execute(args.iter());

    assert!(f.value() && g.value());
}
