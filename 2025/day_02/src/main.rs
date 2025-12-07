const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut acc = 0;
    for range in INPUT.trim().split(",") {
        // make all the inputs into an array
        let range_arr: Vec<u64> = range.split("-").map(|e| e.parse().unwrap()).collect();

        for num in range_arr[0]..range_arr[1] + 1 {
            if has_repeating_digits(num) {
                acc = acc + num;
            }
        }
    }

    println!("{acc}");
}

fn has_repeating_digits(num: u64) -> bool {
    let num_str: String = num.to_string();
    let str_len = num_str.len();

    for pairs in 1..str_len / 2 + 1 {
        let splits: Vec<&str> = num_str
            .as_bytes()
            .chunks(pairs)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect();

        let first_num = splits[0];

        let matching_res: Vec<&&str> = splits.iter().filter(|num| **num == first_num).collect();
        if matching_res.len() == splits.len() {
            println!(
                "{num_str} :: {:?} len({})",
                matching_res,
                matching_res.len()
            );
            return true;
        }
    }

    return false;
}
