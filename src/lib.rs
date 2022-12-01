//! Personal Advent-of-Code Library

/// Advent of Code 2022
pub mod aoc22 {
    /// Day1
    pub fn day1() {
        let input = std::io::stdin().lines().fold(
            String::new(),
            |acc, v| { acc + &v.unwrap() + "\n" },
        );
        let sums = input.split("\n\n").map(
            |v| {
                v.split("\n").filter(
                    |v| { v.len() > 0 }
                ).fold(
                    0,
                    |acc, v| { acc + v.parse::<u64>().unwrap() },
                )
            },
        ).collect::<Vec<u64>>();

        print!("{:?}\n", sums);

        let max = sums.iter().max().unwrap();

        print!("Max: {}\n", max);

        let max3: u64 = sums.iter().fold(
            [0u64, 0u64, 0u64],
            |acc, v| {
                if *v > acc[0] {
                    [*v, acc[0], acc[1]]
                } else if *v > acc[1] {
                    [acc[0], *v, acc[1]]
                } else if *v > acc[2] {
                    [acc[0], acc[1], *v]
                } else {
                    acc
                }
            },
        ).iter().sum();

        print!("Max3: {}\n", max3);
    }
}
