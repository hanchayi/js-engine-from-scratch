use engine::engine::run_script;

#[test]
pub fn check_env() {
    let script = "var a = \"World\";
    function jason() {
      console.log(a);
      return true;
    }
    
    jason();".to_string();
    let res = run_script(script).to_string();
    assert_eq!(res, "true");
}