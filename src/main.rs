extern crate serde_json;

use std::env;
use std::process::{self, Command};
use std::str;

use serde_json::Value;

#[derive(Debug)]
struct Target {
    name: String,
    kind: String,
}

fn parse_targets(metadata: &str) -> Vec<Target> {
    let metadata: Value = serde_json::from_str(&metadata).unwrap();

    let targets = metadata.find("packages").unwrap().as_array().unwrap()[0]
        .find("targets")
        .unwrap()
        .as_array()
        .unwrap();

    targets.into_iter()
        .map(|t| {
            let t = t.as_object().unwrap();

            Target {
                name: t["name"].as_str().unwrap().to_owned(),
                kind: t["kind"]
                        .as_array()
                        .unwrap()[0]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            }
        })
        .collect()
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let targets = {
        let output = Command::new("cargo").arg("metadata").arg("--no-deps").output().unwrap();
        parse_targets(str::from_utf8(&output.stdout).unwrap())
    };

    let num_targets = targets.len();

    for target in &targets {
        let mut args = args.clone();

        if num_targets > 1 {
            match &target.kind[..] {
                "lib" => args.push("--lib".to_owned()),
                "bin" => {
                    args.push("--bin".to_owned());
                    args.push(target.name.to_owned());
                }
                _ => (),
            }
        }

        args = wrap_args(args);

        let mut command = Command::new("cargo");
        command.args(&args);
        let mut child = command.spawn().unwrap_or_else(|e| panic!("{}", e));
        let exit_status = child.wait().unwrap_or_else(|e| panic!("{}", e));

        if let Some(code) = exit_status.code() {
            process::exit(code);
        }
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
