#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from("prime\nprime\nprime\nprime\n7 is a prime\n");
    let exp = String::from_utf8(output.stdout).unwrap();
    assert_eq!(act, exp);
}
