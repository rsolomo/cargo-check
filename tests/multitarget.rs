// static BINARY: &'static str = concat!(env!("CARGO_TARGET_DIR"), env!("CARGO_PKG_NAME"));

use std::env;
use std::process::Command;
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

#[test]
fn multiple_targets_2() {
    let multiple_target_fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/multitarget");

    let status = cargo_check()
        .args(&["check", "--", "--cfg", "wrong_bin"])
        .current_dir(multiple_target_fixture)
        .status()
        .unwrap();

    assert!(!status.success());
}

// The test ensures that `cargo check` bails after first failing compilation.
#[test]
fn multiple_targets_3() {
    let multiple_target_fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/multitarget");

    let output = cargo_check()
        .args(&["check", "--", "--cfg", "wrong_bin", "--cfg", "wrong_lib"])
        .current_dir(multiple_target_fixture)
        .output()
        .unwrap()
        .stderr;

    let output = String::from_utf8(output).unwrap();

    // This is the error message from compiling `--lib`
    assert!(output.contains("no associated item"));
    // This is the error message from compiling `--bin multitarget`.
    // This should not be present as `cargo check` should bail after first
    // failure.
    assert!(!output.contains("unresolved name"));
}

#[test]
fn multiple_targets_4() {
    let multiple_target_fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/multitarget");

    let status = cargo_check()
        .args(&["check", "--release"])
        .current_dir(multiple_target_fixture)
        .status()
        .unwrap();

    assert!(status.success());
}
