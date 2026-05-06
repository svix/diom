use std::process::Command;

#[test]
fn binary_starts_without_panic() {
    let bin = env!("CARGO_BIN_EXE_diom-operator");
    let output = Command::new(bin).arg("--print-crd").output().unwrap();

    assert!(output.status.success());
}
