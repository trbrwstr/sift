use std::process::Command;

#[test]
fn runs_basic_analyze() {
    let output = Command::new("cargo")
        .args(["run", "-p", "logforge-cli", "--", "stats", "examples/sample.log"])
        .output()
        .expect("failed to run");

    assert!(output.status.success());
}