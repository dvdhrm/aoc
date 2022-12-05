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

    /// Day3
    pub fn day3() {
        let input = std::io::BufRead::lines(std::io::stdin().lock()).map(
            |v| {
                let line = v.unwrap();
                let n_line = line.len();

                assert!(n_line > 0);
                assert!((n_line % 2) == 0);

                let (l1, l2) = line.split_at(n_line / 2);
                let h1 = std::collections::hash_set::HashSet::<u8>::from_iter(l1.bytes());
                let h2 = std::collections::hash_set::HashSet::<u8>::from_iter(l2.bytes());

                *h1.intersection(&h2).next().unwrap()
            },
        ).collect::<Vec<u8>>();

        let score: u64 = input.iter().map(
            |v| {
                if *v >= b'a' && *v <= b'z' {
                    (*v - b'a' + 1) as u64
                } else {
                    assert!(*v >= b'A' && *v <= b'Z');
                    (*v - b'A' + 27) as u64
                }
            },
        ).sum();

        print!("{:?}\n", score);
    }

    /// Day3_2
    pub fn day3_2() {
        let input = std::io::BufRead::lines(std::io::stdin().lock()).map(|v| v.unwrap()).collect::<Vec<String>>();
        let chunked = input.chunks(3).map(
            |v| {
                let h1 = std::collections::hash_set::HashSet::<u8>::from_iter(v[0].bytes());
                let h2 = std::collections::hash_set::HashSet::<u8>::from_iter(v[1].bytes());
                let h3 = std::collections::hash_set::HashSet::<u8>::from_iter(v[2].bytes());
                let t = h1.intersection(&h2).copied().collect::<std::collections::hash_set::HashSet<u8>>();
                *t.intersection(&h3).next().unwrap()
            }
        ).collect::<Vec<u8>>();

        let score: u64 = chunked.iter().map(
            |v| {
                if *v >= b'a' && *v <= b'z' {
                    (*v - b'a' + 1) as u64
                } else {
                    assert!(*v >= b'A' && *v <= b'Z');
                    (*v - b'A' + 27) as u64
                }
            },
        ).sum();

        print!("{:?}\n", score);
    }

    /// Day4
    pub fn day4() {
        let input = std::io::BufRead::lines(std::io::stdin().lock()).map(
            |v| {
                let line = v.unwrap();
                let pair = line.split(",").collect::<Vec<&str>>();
                assert!(pair.len() == 2);

                let p0 = pair[0].split("-").collect::<Vec<&str>>();
                let p1 = pair[1].split("-").collect::<Vec<&str>>();
                assert!(p0.len() == 2);
                assert!(p1.len() == 2);

                (
                    p0[0].parse::<u64>().unwrap(),
                    p0[1].parse::<u64>().unwrap(),
                    p1[0].parse::<u64>().unwrap(),
                    p1[1].parse::<u64>().unwrap(),
                )
            },
        ).collect::<Vec<(u64, u64, u64, u64)>>();

        let contained = input.iter().filter(
            |&v| {
                (v.0 >= v.2 && v.0 <= v.3 && v.1 >= v.2 && v.1 <= v.3) ||
                (v.2 >= v.0 && v.2 <= v.1 && v.3 >= v.0 && v.3 <= v.1)
            }
        ).collect::<Vec<&(u64, u64, u64, u64)>>();

        let overlapped = input.iter().filter(
            |&v| {
                !(v.1 < v.2 || v.3 < v.0)
            }
        ).collect::<Vec<&(u64, u64, u64, u64)>>();

        print!("{:?}\n", contained.len());
        print!("{:?}\n", overlapped.len());
    }
}
