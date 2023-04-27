#[cfg(target_os = "linux")]
#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from_utf8(output.stdout).unwrap();
    let exp = String::from("TODO\n");
    assert_eq!(act, exp);
}

#[cfg(not(target_os = "linux"))]
#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from_utf8(output.stdout).unwrap();
    let exp = String::from("Skip\n");
    assert_eq!(act, exp);
}
