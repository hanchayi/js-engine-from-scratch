use engine::{engine::run_script, js::{value::ValueData, string::_create}};


#[test]
fn check_new_string() {
    let script = "var a = new String(\"jason\");
    a.length;".to_string();
    let res = run_script(script);
    assert_eq!(res.to_string(), "5");
}

#[test]
fn check_string_length() {
    let script = "var a =  'jabra888';
    a.length;".to_string();
    let res = run_script(script);
    assert_eq!(res.to_string(), "8");
}

#[test]
fn check_char_at() {
    let script = "var a = new String(\"test\");
    a.charAt(1);".to_string();
    let res = run_script(script);
    assert_eq!(res.to_string(), "e");
}

