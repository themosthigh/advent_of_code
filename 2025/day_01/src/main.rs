const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut dial = 50;
    let mut acc = 0;
    let mut acc_abs = 0;

    let instructions = get_instructions();

    for (index, (command, rotation)) in instructions.iter().enumerate() {
        let initial = dial;

        dial = match command {
            Command::Increment => dial + rotation,
            Command::Decrement => dial - rotation,
            Command::Unknown => dial,
        };

        let _initial = ((initial as f32) / 100.0).floor();
        let _res = (dial as f32 / 100.0).floor();
        let diff = (_res - _initial).abs() as i32;

        println!("{index}: {initial}/{rotation}={dial} ::: {diff} as {_res}-{_initial}");

        acc_abs = acc_abs + diff;

        if dial.abs() % 100 == 0 {
            acc = acc + 1;
        }
    }

    println!("acc: {acc}, abs: {acc_abs} => {}", acc + acc_abs);
}

enum Command {
    Decrement,
    Increment,
    Unknown,
}

fn get_instructions() -> Vec<(Command, i32)> {
    let mut instructions: Vec<(Command, i32)> = vec![];

    for instruction in INPUT.split("\n") {
        if instruction == "" {
            continue;
        }

        let _in: Vec<String> = instruction.split("").map(|e| e.to_string()).collect();

        let command = if _in[1] == "L" {
            Command::Decrement
        } else if _in[1] == "R" {
            Command::Increment
        } else {
            Command::Unknown
        };

        let __in = _in[2..].join("");
        let rotation = __in.parse().unwrap();

        instructions.push((command, rotation));
    }

    return instructions;
}
