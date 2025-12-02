const fs = require("fs");

const input = fs.readFileSync("../input.txt").toString();

let dial = 50;
let acc = 0;

for (line of input.split("\n")) {
  if (!line) continue;
  const operator = line[0];
  const value = parseInt(line.split("").slice(1).join(""));

  if (operator == "L") {
    dial = (100 + dial - value) % 100;
  } else if (operator == "R") {
    dial = (dial + value) % 100;
  }

  if (dial % 100 === 0) {
    acc++;
  }
}

console.log({ acc });
