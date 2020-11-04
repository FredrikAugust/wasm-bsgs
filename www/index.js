import * as wasm from "wasm-bsgs";

// Call once for error handling.
wasm.init();

const modulo = document.querySelector("#modulo");
const alpha = document.querySelector("#alpha");
const beta = document.querySelector("#beta");

const solve = document.querySelector("#solve");

const results = document.querySelector("#result");

solve.addEventListener("click", () => {
  console.log(
    `Solving for modulo=${modulo.value}, alpha=${alpha.value}, beta=${beta.value}.`
  );

  const start = new Date();

  const result = wasm.calculate(modulo.value, alpha.value, beta.value);

  const end = new Date();

  results.textContent = `${result} (solved in ${
    end.getTime() - start.getTime()
  }ms)`;
});
