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

    /// Day2
    pub fn day2() {
        let input = std::io::BufRead::lines(std::io::stdin().lock()).map(
            |v| {
                let line = v.unwrap();
                let parts = line.split(" ").collect::<Vec<&str>>();
                assert!(parts.len() == 2);
                assert!(parts[0].len() == 1);
                assert!(parts[1].len() == 1);
                [
                    parts[0].chars().nth(0).unwrap(),
                    parts[1].chars().nth(0).unwrap(),
                ]
            },
        ).collect::<Vec<[char; 2]>>();

        let score = input.iter().fold(
            0,
            |acc, v| {
                acc + match v[0] {
                    'A' => match v[1] {
                        'X' => 3 + 1,
                        'Y' => 6 + 2,
                        'Z' => 0 + 3,
                        _ => unreachable!(),
                    },
                    'B' => match v[1] {
                        'X' => 0 + 1,
                        'Y' => 3 + 2,
                        'Z' => 6 + 3,
                        _ => unreachable!(),
                    },
                    'C' => match v[1] {
                        'X' => 6 + 1,
                        'Y' => 0 + 2,
                        'Z' => 3 + 3,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            },
        );

        let score2 = input.iter().fold(
            0,
            |acc, v| {
                acc + match v[0] {
                    'A' => match v[1] {
                        'X' => 0 + 3,
                        'Y' => 3 + 1,
                        'Z' => 6 + 2,
                        _ => unreachable!(),
                    },
                    'B' => match v[1] {
                        'X' => 0 + 1,
                        'Y' => 3 + 2,
                        'Z' => 6 + 3,
                        _ => unreachable!(),
                    },
                    'C' => match v[1] {
                        'X' => 0 + 2,
                        'Y' => 3 + 3,
                        'Z' => 6 + 1,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            },
        );

        print!("{:?}\n", score);
        print!("{:?}\n", score2);
    }
}
