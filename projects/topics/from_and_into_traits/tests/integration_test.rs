#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from_utf8(output.stdout).unwrap();
    let exp = String::from("mystruct.0 is 1\ni is 1\n");
    assert_eq!(act, exp);
}
