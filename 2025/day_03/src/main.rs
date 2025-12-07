const INPUT: &str = include_str!("../input.test.txt");

fn main() {
    let mut acc: u32 = 0;
    for line in INPUT.split("\n") {
        if line == "" {
            continue;
        }

        let mut joins: Vec<u32> = vec![];
        let line_vec: Vec<&str> = line.trim().split("").filter(|e| *e != "").collect();

        for (i, char_i) in line_vec[0..line_vec.len() - 1].iter().enumerate() {
            for (_j, char_j) in line_vec[i + 1..line_vec.len()].iter().enumerate() {
                joins.push((char_i.to_string() + char_j).parse().unwrap());
            }
        }

        joins.sort_by(|a, b| b.cmp(&a));
        acc = acc + joins[0];
    }
    println!("{acc}");
}
