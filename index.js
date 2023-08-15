const rust = import("./pkg/engine");

rust.then(m => {
  window.evaluate = m.evaluate;
  m.evaluate('var a = "hello"; a;');
});