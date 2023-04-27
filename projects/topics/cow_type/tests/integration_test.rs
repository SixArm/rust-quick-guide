#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let exp = String::from("Borrowed\nOwned\n");
    let act = String::from_utf8(output.stdout).unwrap();
    assert_eq!(act, exp);
}
