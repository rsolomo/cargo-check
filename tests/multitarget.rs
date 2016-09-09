// static BINARY: &'static str = concat!(env!("CARGO_TARGET_DIR"), env!("CARGO_PKG_NAME"));

use std::env;
use std::process::{Command, Stdio};
use std::path::PathBuf;

fn cargo_check() -> Command {
    Command::new(env::current_exe().unwrap().parent().unwrap().join("cargo-check"))
}

#[test]
fn self_test() {
    let status = cargo_check()
        .arg("check")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn multiple_targets() {
    let multiple_target_fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/multitarget");

    let status = cargo_check()
        .arg("check")
        .current_dir(multiple_target_fixture)
        .status()
        .unwrap();

    assert!(status.success());
}
