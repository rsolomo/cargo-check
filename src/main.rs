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
    let targets = {
        let output = Command::new("cargo").arg("metadata").arg("--no-deps").output().unwrap();
        parse_targets(str::from_utf8(&output.stdout).unwrap())
    };

    for target in &targets {
        let mut args = vec!["rustc".to_owned()];

        match &target.kind[..] {
            "lib" => args.push("--lib".to_owned()),
            "bin" => {
                args.push("--bin".to_owned());
                args.push(target.name.to_owned())
            }
            _ => (),
        };

        args.extend(env::args().skip(2).take_while(|arg| arg != "--"));
        args.push("--".to_owned());
        args.extend(env::args().skip(2).skip_while(|arg| arg != "--").skip(1));
        args.push("-Zno-trans".to_owned());

        let status = Command::new("cargo").args(&args).status().unwrap();

        // If the command didn't execute successfully, exit this process with
        // the same code, defaulting to an arbitrary error exit code (1) if
        // `status.code()` returns None.
        if !status.success() {
            process::exit(status.code().unwrap_or(1));
        }
    }
}
