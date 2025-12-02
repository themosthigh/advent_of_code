const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut dial = 50;
    let mut acc = 0;

    let instructions = get_instructions();

    for (command, rotation) in instructions {
        dial = match command {
            Command::Increment => (dial + rotation) % 100,
            Command::Decrement => (100 + dial - rotation) % 100,
            Command::Unknown => dial,
        };

        if dial % 100 == 0 {
            acc = acc + 1;
        }
    }

    println!("{acc}");
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
        println!("{__in}");
        let rotation = __in.parse().unwrap();

        instructions.push((command, rotation));
    }

    return instructions;
}
