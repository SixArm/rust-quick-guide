#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from("Captures({0: Some(\"1b\"), 1: Some(\"1\"), 2: Some(\"b\")})\n");
    let exp = String::from_utf8(output.stdout).unwrap();
    assert_eq!(act, exp);
}
