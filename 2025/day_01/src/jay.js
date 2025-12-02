const fs = require("fs");

const input = fs.readFileSync("../input.txt").toString();

let dial = 50;
let acc = 0;
let acc_abs = 0;

for (line of input.split("\n")) {
  const previous = dial;

  if (!line) continue;
  const operator = line[0];
  const value = parseInt(line.split("").slice(1).join(""));

  if (operator == "L") {
    dial = dial - value;
  } else if (operator == "R") {
    dial = dial + value;
  }

  if (dial % 100 === 0) {
    acc++;
  }

  const diff = Math.abs(dial - previous);
  const diff_base = Math.floor(diff / 100);
  console.log({ previous, dial, diff_base });
  acc_abs += diff_base;
}

console.log({ acc, dial, acc_abs, res: acc + acc_abs });
