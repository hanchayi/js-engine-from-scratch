const rust = import("./pkg/engine");

rust.then(m => {
    window.evaluate = m.evaluate;
    let button = document.querySelector("button");
    button.addEventListener("click", clickHandler);
});

function clickHandler(evt) {
    let text = document.querySelector("textarea").value;
    let p = document.querySelector("p.output");
    let t0 = performance.now();
    let result = window.evaluate(text);
    let t1 = performance.now();
    p.textContent = `> ${result}`;
    console.log(result);
  }