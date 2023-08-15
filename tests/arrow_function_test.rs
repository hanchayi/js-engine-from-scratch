use engine::engine::run_script;

#[test]
pub fn check_econst() {
    let script = "function getConf() {
        let jase = () => {
          console.log(\"Hello\");
        };
      
        jase();
      }
      
      getConf();".to_string();
    let res = run_script(script).to_string();
}