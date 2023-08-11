use engine::engine::run_script;


#[test]
fn check_new_string() {
    let script = "var a = new String(\"jason\");
    a;".to_string();
    let res = run_script(script);
    println!("{}", res)
}

#[test]
fn check_string_length() {
    let script = "var a =  'jabra888';
    a.length;".to_string();
    let res = run_script(script);
    println!("{}", res);
    assert_eq!(res.to_string(), "8");
}