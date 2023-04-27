#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from_utf8(output.stdout).unwrap().to_owned();
    let exp = String::from("1a\n1b\n1c\n2a\n2b\n2c\n3a\n3b\n3c\n['a', 'b']\n['a', 'c']\n['b', 'a']\n['b', 'c']\n['c', 'a']\n['c', 'b']\n\"a, b, c\"\n");
    assert_eq!(act, exp);
}
