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

fn wrap_args<T, I>(it: I) -> Vec<String>
    where T: AsRef<str>,
          I: IntoIterator<Item = T>
{

    let it = it.into_iter();
    let mut args = vec!["rustc".to_owned()];
    let mut has_double_hyphen = false;

    for arg in it.skip(2) {
        let arg = arg.as_ref().to_owned();
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
    let args = ["/usr/local/bin/cargo-check", "check", "-h"];
    let actual = wrap_args(&args);
    let expected = ["rustc", "-h", "--", "-Zno-trans"];
    assert_eq!(actual, expected);
}

#[test]
fn wrap_args_2() {
    let args = ["/usr/local/bin/cargo-check", "check", "--", "-Zverbose"];
    let actual = wrap_args(&args);
    let expected = ["rustc", "--", "-Zverbose", "-Zno-trans"];
    assert_eq!(actual, expected);
}
