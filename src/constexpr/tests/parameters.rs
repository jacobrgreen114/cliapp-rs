use super::*;

#[test]
fn parameter_parsing_1() {
    let f = ParameterValue::new();
    let g = ParameterValue::new();

    let parameters = &[
        Parameter::build()
            .with_long_name("f")
            .with_description("A parameter")
            .with_parameter(&f)
            .build(),
        Parameter::build()
            .with_short_name("g")
            .with_description("A gparameter")
            .with_parameter(&g)
            .build(),
    ];

    let app: Application = Application::build()
        .with_parameters(parameters)
        .with_command(&default_command)
        .build();

    let args: [&str; 0] = [];

    app.execute(args.iter());

    assert!(f.value().is_none() && g.value().is_none());
}

#[test]
fn parameter_parsing_2() {
    let f = ParameterValue::new();
    let g = ParameterValue::new();

    let parameters = &[
        Parameter::build()
            .with_long_name("fparam")
            .with_description("A parameter")
            .with_parameter(&f)
            .build(),
        Parameter::build()
            .with_short_name("gparam")
            .with_description("A gparameter")
            .with_parameter(&g)
            .build(),
    ];

    let app: Application = Application::build()
        .with_parameters(parameters)
        .with_command(&default_command)
        .build();

    let args = &["--fparam=hello"];

    app.execute(args.iter());

    assert!(f.value().is_some() && f.value().unwrap() == "hello" && g.value().is_none());
}
