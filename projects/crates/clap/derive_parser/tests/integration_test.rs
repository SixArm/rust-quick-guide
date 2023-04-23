#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(!output.status.success());
    let act = String::from_utf8(output.stderr).unwrap();
    let mut lines = act.lines();
    lines.next(); // Skip compiler output about build
    lines.next(); // Skip compiler output about run
    let act = lines.collect::<Vec<&str>>().join("\n");
    let exp = String::from("error: the following required arguments were not provided:\n  --name <NAME>\n  --age <AGE>\n\nUsage: clap-derive-parser --name <NAME> --age <AGE>\n\nFor more information, try '--help'.");
    assert_eq!(act, exp);
}
