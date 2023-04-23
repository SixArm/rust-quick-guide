#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from("1\n4\n9\n16\n25\n36\n49\n64\n2\n4\n6\n8\n");
    let exp = String::from_utf8(output.stdout).unwrap();
    assert_eq!(act, exp);
}


