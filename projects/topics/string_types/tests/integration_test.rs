#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from_utf8(output.stdout).unwrap();
    let exp = String::from("\"foo\" \"foo\" \"foo\" \"foo\" \"foo\" \"foo\" \"foo\" \"foo\"\n");
    assert_eq!(act, exp);
}
