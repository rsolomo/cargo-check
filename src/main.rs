#[cfg(not(test))]
fn main() {
    use std::env;
    use std::process::{self, Command};

    let args = wrap_args(env::args());
    let mut command = Command::new("cargo");
    command.args(&args);
    let mut child = command.spawn().unwrap_or_else(|e| panic!("{}", e));
    let exit_status = child.wait().unwrap_or_else(|e| panic!("{}", e));

    if let Some(code) = exit_status.code() {
        process::exit(code);
    }
}

fn wrap_args<I: Iterator<Item=String>>(it: I) -> Vec<String> {
    let mut args = vec!["rustc".to_owned()];
    let mut has_double_hyphen = false;
    for arg in it.skip(2) {
        has_double_hyphen |= &arg == "--";
        args.push(arg);
    }

    if !has_double_hyphen {
        args.push("--".to_owned());
    }
    args.push("-Zno-trans".to_owned());
    args
}

#[test]
fn wrap_args_1() {
    let args = vec![
        "/usr/local/bin/cargo-check".to_owned(),
        "check".to_owned(),
        "-h".to_owned()
    ];
    let actual = wrap_args(args.into_iter());
    let expected = vec![
        "rustc".to_owned(),
        "-h".to_owned(),
        "--".to_owned(),
        "-Zno-trans".to_owned()
    ];
    assert_eq!(actual, expected);
}

#[test]
fn wrap_args_2() {
    let args = vec![
        "/usr/local/bin/cargo-check".to_owned(),
        "check".to_owned(),
        "--".to_owned(),
        "-Zverbose".to_owned()
    ];
    let actual = wrap_args(args.into_iter());
    let expected = vec![
        "rustc".to_owned(),
        "--".to_owned(),
        "-Zverbose".to_owned(),
        "-Zno-trans".to_owned()
    ];
    assert_eq!(actual, expected);
}
