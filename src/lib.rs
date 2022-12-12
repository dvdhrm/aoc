//! Personal Advent-of-Code Library

/// Advent of Code 2022
pub mod aoc22 {
    use std::collections::BTreeMap;

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

    /// Day5
    pub fn day5() {
        let input = std::io::BufRead::lines(std::io::stdin().lock()).fold(
            String::new(),
            |acc, v| { acc + &v.unwrap() + "\n" },
        );
        let mut input_split = input.split("\n\n");
        let init = input_split.next().unwrap();
        let inst = input_split.next().unwrap();

        let mut init_lines = init.split("\n").map(
            |x| {
                (x.to_owned() + " ").as_bytes().chunks(4).map(
                    |x| {
                        x[1]
                    },
                ).collect::<Vec<u8>>()
            },
        ).collect::<Vec<Vec<u8>>>();
        init_lines.pop();

        let mut stacks = Vec::<Vec<u8>>::new();
        for layer in init_lines.iter().rev() {
            if layer.len() >= stacks.len() {
                stacks.resize(layer.len(), Vec::<u8>::new());
            }
            for (idx, entry) in layer.iter().enumerate() {
                if *entry != 32 {
                    stacks[idx].push(*entry);
                }
            }
        }

        let mut inst_lines = inst.split("\n").collect::<Vec<&str>>();
        inst_lines.pop();
        let inst_args = inst_lines.iter().map(
            |v| {
                let s = v.split(" ").collect::<Vec<&str>>();
                (
                    s[1].parse::<usize>().unwrap(),
                    s[3].parse::<usize>().unwrap(),
                    s[5].parse::<usize>().unwrap(),
                )
            },
        ).collect::<Vec<(usize, usize, usize)>>();

        let mut stacks2 = stacks.clone();

        for i in (&inst_args).iter() {
            for _ in 0..i.0 {
                let x = stacks[i.1 - 1].pop().unwrap();
                stacks[i.2 - 1].push(x);
            }
        }

        for i in inst_args {
            print!("INST: {:?}\n", i);
            let s = &mut stacks2[i.1 - 1];
            print!("LEN: {:?}\n", s.len());
            assert!(s.len() >= i.0);
            let dropped = s.drain((s.len() - i.0)..(s.len())).collect::<Vec<u8>>();
            for j in dropped {
                stacks2[i.2 - 1].push(j);
            }
        }

        let top = String::from_utf8(
            stacks.iter().map(
                |v| v.last().unwrap(),
            ).copied().collect::<Vec<u8>>(),
        ).unwrap();
        let top2 = String::from_utf8(
            stacks2.iter().map(
                |v| v.last().unwrap(),
            ).copied().collect::<Vec<u8>>(),
        ).unwrap();

        print!("{:?}\n", top);
        print!("{:?}\n", top2);
    }

    /// Day6
    pub fn day6() {
        let input = std::io::read_to_string(std::io::stdin().lock()).unwrap();
        let mut shift = ['0'; 14];
        let mut n_shift: usize = 0;

        let test = |shift: &[char; 14]| {
            for (idx, value) in shift.iter().enumerate() {
                for dup in &shift[(idx + 1)..] {
                    if dup == value {
                        return true;
                    }
                }
            }

            return false;
        };

        let garb = input.chars().map_while(
            |v| {
                if n_shift < shift.len() || test(&shift) {
                    shift.rotate_left(1);
                    shift[shift.len() - 1] = v;
                    n_shift += 1;
                    Some(v)
                } else {
                    None
                }
            }
        ).collect::<Vec<char>>();

        print!("{:?}\n", garb.len());
    }

    /// Day7
    pub fn day7() {
        let input = std::io::read_to_string(std::io::stdin().lock()).unwrap();

        let cmds = input.split("\n$ ").skip(1).map(
            |v| {
                let mut lines = v.split("\n");
                let cmd = lines.next().unwrap().split(" ").collect::<Vec<&str>>();
                let args = lines.map(|v| v.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

                (cmd, args)
            },
        ).collect::<Vec<(Vec<&str>, Vec<Vec<&str>>)>>();

        let mut data: BTreeMap<String, u64> = BTreeMap::new();
        let mut cwd: Vec<&str> = Vec::new();

        data.insert("/".to_string(), 0);

        for (cmd, res) in cmds {
            if cmd[0] == "cd" {
                if cmd[1] == ".." {
                    cwd.pop();
                } else {
                    cwd.push(cmd[1]);
                }
            } else if cmd[0] == "ls" {
                for entry in res {
                    if entry[0].len() == 0 {
                        continue;
                    }

                    let name = if cwd.len() == 0 {
                        "/".to_string() + entry[1]
                    } else {
                        "/".to_string() + &cwd.as_slice().join("/") + "/" + entry[1]
                    };

                    if entry[0] == "dir" {
                        assert!(data.insert(name, 0).is_none());
                    } else {
                        let size = entry[0].parse::<u64>().unwrap();

                        *data.get_mut("/").unwrap() += size;

                        for i in 0..cwd.len() {
                            let name = if cwd.len() == 0 {
                                "/".to_string()
                            } else {
                                "/".to_string() + &cwd.as_slice()[0..i+1].join("/")
                            };
                            *data.get_mut(&name).unwrap() += size;
                        }
                    }
                }
            } else {
                unreachable!();
            }
        }

        let sum: u64 = data.values().filter(
            |v| **v <= 100000,
        ).map(|v| *v).sum();

        print!("{:?}\n", sum);

        let rsize = data["/"];
        let diff = 30000000 - (70000000 - rsize);

        let mut ssize = rsize;
        for &dir in data.values() {
            if dir >= diff && dir <= ssize {
                ssize = dir;
            }
        }

        print!("{:?}\n", ssize);
    }

    /// Day8
    pub fn day8() {
        let input = std::io::read_to_string(std::io::stdin().lock()).unwrap();

        // Parse input into u8-grid.
        let grid = input.split("\n").filter(|v| v.len() > 0).map(
            |v| v.chars().map(
                |v| v.to_digit(10).unwrap() as u8,
            ).collect::<Vec<u8>>(),
        ).collect::<Vec<Vec<u8>>>();
        let n_rows = grid.len();
        let n_cols= grid[0].len();

        // Create state-grid.
        let mut state: Vec::<Vec<bool>> = Vec::new();
        state.resize(n_rows, Vec::new());
        for v in state.iter_mut() {
            v.resize(n_cols, false);
        }

        // Calculate visible trees.
        let mut height: isize;
        let mut value: isize;

        for row in 0..n_rows {
            // row forwards
            height = -1;
            for col in 0..n_cols {
                value = grid[row][col] as isize;
                if value > height {
                    state[row][col] = true;
                    height = value;
                }
            }

            // row backwards
            height = -1;
            for col in (0..n_cols).rev() {
                value = grid[row][col] as isize;
                if value > height {
                    state[row][col] = true;
                    height = value;
                }
            }
        }

        for col in 0..n_cols {
            // column forwards
            height = -1;
            for row in 0..n_rows {
                value = grid[row][col] as isize;
                if value > height {
                    state[row][col] = true;
                    height = value;
                }
            }

            // column backwards
            height = -1;
            for row in (0..n_rows).rev() {
                value = grid[row][col] as isize;
                if value > height {
                    state[row][col] = true;
                    height = value;
                }
            }
        }

        let visible: usize = state.iter().map(
            |v| v.iter().map(|v| if *v { 1 } else { 0 },).sum::<usize>(),
        ).sum();

        print!("{:?}\n", visible);

        // Create second state-grid.
        let mut state2: Vec::<Vec<usize>> = Vec::new();
        state2.resize(n_rows, Vec::new());
        for v in state2.iter_mut() {
            v.resize(n_cols, 1);
        }

        // Calculate scenic scores.
        for row in 0..n_rows {
            for col in 0..n_cols {
                height = grid[row][col] as isize;

                if row == 0 || row == n_rows-1 || col == 0 || col == n_cols-1 {
                    state2[row][col] = 0;
                    continue;
                }

                // left view
                for ncol in (0..col).rev() {
                    value = grid[row][ncol] as isize;
                    if value >= height || ncol == 0 {
                        state2[row][col] *= col - ncol;
                        break;
                    }
                }

                // up view
                for nrow in (0..row).rev() {
                    value = grid[nrow][col] as isize;
                    if value >= height || nrow == 0 {
                        state2[row][col] *= row - nrow;
                        break;
                    }
                }

                // right view
                for ncol in col+1..n_cols {
                    value = grid[row][ncol] as isize;
                    if value >= height || ncol == n_cols-1 {
                        state2[row][col] *= ncol - col;
                        break;
                    }
                }

                // down view
                for nrow in row+1..n_rows {
                    value = grid[nrow][col] as isize;
                    if value >= height || nrow == n_rows-1 {
                        state2[row][col] *= nrow - row;
                        break;
                    }
                }
            }
        }

        let scenic_max = state2.iter().map(
            |v| v.iter().max().unwrap(),
        ).max().unwrap();

        print!("{:?}\n", scenic_max);
    }

    /// Day9
    pub fn day9() {
        let input = std::io::read_to_string(std::io::stdin().lock()).unwrap();

        let inst = input.split("\n").filter(|v| v.len() > 0).map(
            |v| {
                let fields = v.split(" ").collect::<Vec<&str>>();
                assert!(fields.len() == 2);
                assert!(fields[0].len() == 1);

                (fields[0].chars().next().unwrap(), fields[1].parse::<i32>().unwrap())
            },
        ).collect::<Vec<(char, i32)>>();

        let mut grid: BTreeMap<(i32, i32), bool> = BTreeMap::new();
        let mut head: (i32, i32) = (0, 0);
        let mut tail: (i32, i32) = (0, 0);

        grid.insert(tail, true);

        for &(dir, len) in inst.iter() {
            for _ in 0..len {
                match dir {
                    'U' => head = (head.0, head.1 + 1),
                    'R' => head = (head.0 + 1, head.1),
                    'D' => head = (head.0, head.1 - 1),
                    'L' => head = (head.0 - 1, head.1),
                    _ => unreachable!(),
                }

                if (head.0 - tail.0).abs() > 1 {
                    tail = (
                        tail.0 + (head.0 - tail.0) / 2,
                        head.1,
                    );
                } else if (head.1 - tail.1).abs() > 1 {
                    tail = (
                        head.0,
                        tail.1 + (head.1 - tail.1) / 2,
                    );
                }

                grid.insert(tail, true);
            }
        }

        print!("{:?}\n", grid.len());

        const N: usize = 10;
        let mut knots: [(i32, i32); N] = [(0, 0); N];
        let mut grid2: BTreeMap<(i32, i32), bool> = BTreeMap::new();

        grid2.insert(knots[N-1], true);

        for &(dir, len) in inst.iter() {
            for _ in 0..len {
                match dir {
                    'U' => knots[0] = (knots[0].0, knots[0].1 + 1),
                    'R' => knots[0] = (knots[0].0 + 1, knots[0].1),
                    'D' => knots[0] = (knots[0].0, knots[0].1 - 1),
                    'L' => knots[0] = (knots[0].0 - 1, knots[0].1),
                    _ => unreachable!(),
                }

                for idx in 1..N {
                    let head = knots[idx-1];
                    let tail = &mut knots[idx];

                    if (head.0 - tail.0).abs() > 1 &&
                       (head.1 - tail.1).abs() > 1 {
                        *tail = (
                            tail.0 + (head.0 - tail.0) / 2,
                            tail.1 + (head.1 - tail.1) / 2,
                        );
                    } else if (head.0 - tail.0).abs() > 1 {
                        *tail = (
                            tail.0 + (head.0 - tail.0) / 2,
                            head.1,
                        );
                    } else if (head.1 - tail.1).abs() > 1 {
                        *tail = (
                            head.0,
                            tail.1 + (head.1 - tail.1) / 2,
                        );
                    }
                }

                grid2.insert(knots[N-1], true);
            }
        }

        print!("{:?}\n", grid2.len());
    }

    /// Day10
    pub fn day10() {
        let input = std::io::read_to_string(std::io::stdin().lock()).unwrap();

        let inst = input.split("\n").filter(|v| v.len() > 0).map(
            |v| {
                let cmd = v.split(" ").collect::<Vec<&str>>();
                assert!(cmd.len() > 0);
                if cmd[0] == "noop" {
                    assert!(cmd.len() == 1);
                    (0, 0)
                } else if cmd[0] == "addx" {
                    assert!(cmd.len() == 2);
                    (1, cmd[1].parse::<i8>().unwrap())
                } else {
                    unreachable!();
                }
            },
        ).collect::<Vec<(usize, i8)>>();

        let mut x: i64 = 1;
        let mut cycle: usize = 0;
        let mut intr: usize = 20;
        let mut sum: i64 = 0;

        for (code, value) in inst.iter() {
            let new_x;
            let new_cycle;

            match code {
                0 => {
                    new_cycle = cycle + 1;
                    new_x = x;
                },
                1 => {
                    new_cycle = cycle + 2;
                    new_x = x + *value as i64;
                },
                _ => unreachable!(),
            }

            if intr <= new_cycle {
                sum = sum + (intr as i64) * x;
                intr = intr + 40;
            }

            x = new_x;
            cycle = new_cycle;
        }

        print!("{:?}\n", sum);

        let mut grid: [[bool; 40]; 6] = [[false; 40]; 6];

        x = 1;
        cycle = 0;

        for (code, value) in inst.iter() {
            let new_x;
            let new_cycle;

            match code {
                0 => {
                    new_cycle = cycle + 1;
                    new_x = x;
                },
                1 => {
                    new_cycle = cycle + 2;
                    new_x = x + *value as i64;
                },
                _ => unreachable!(),
            }

            while cycle < new_cycle {
                let h = cycle / 40;
                let w = cycle % 40;
                cycle = cycle + 1;

                assert!(h < 6);
                grid[h][w] = w as i64 >= x - 1 && w as i64 <= x + 1;

                if grid[h][w] {
                    print!("#");
                } else {
                    print!(" ");
                }
                if w == 39 {
                    print!("\n");
                }
            }

            x = new_x;
        }
    }

    /// Day11
    pub fn day11() {
        let input = std::io::read_to_string(std::io::stdin().lock()).unwrap();

        let inst = input.split("\n\n").map(
            |v| {
                let data = v.split("\n")
                    .filter(|v| v.len() > 0)
                    .skip(1)
                    .map(|v| v.split(":").skip(1).next().unwrap().trim())
                    .collect::<Vec<&str>>();

                assert_eq!(data.len(), 5);

                (
                    data[0].split(", ").map(|v| v.parse::<u128>().unwrap()).collect::<Vec<u128>>(),
                    data[1].strip_prefix("new = ").unwrap(),
                    data[2].strip_prefix("divisible by ").unwrap().parse::<u128>().unwrap(),
                    data[3].strip_prefix("throw to monkey ").unwrap().parse::<usize>().unwrap(),
                    data[4].strip_prefix("throw to monkey ").unwrap().parse::<usize>().unwrap(),
                )
            },
        ).collect::<Vec<(Vec<u128>, &str, u128, usize, usize)>>();

        let mut inv: Vec<Vec<u128>> = Vec::new();
        let mut cnt: Vec<u128> = Vec::new();

        for (items, _, _, _, _) in inst.iter() {
            inv.push(items.clone());
            cnt.push(0);
        }

        for _ in 0..20 {
            for (idx, mon) in inst.iter().enumerate() {
                inv.push(Vec::new());
                let items = inv.swap_remove(idx);

                for item in items {
                    let mut w = item;

                    w = match mon.1 {
                        "old * 13" => w * 13,
                        "old + 4" => w + 4,
                        "old * 11" => w * 11,
                        "old + 8" => w + 8,
                        "old * old" => w * w,
                        "old + 5" => w + 5,
                        "old + 1" => w + 1,
                        "old + 3" => w + 3,
                        _ => unreachable!(),
                    };

                    w = w / 3;

                    let target = if w % mon.2 == 0 {
                        mon.3
                    } else {
                        mon.4
                    };

                    inv[target].push(w);
                    cnt[idx] = cnt[idx] + 1;
                }
            }
        }

        cnt.sort();
        print!("{:#?}\n", cnt[cnt.len() - 1] * cnt[cnt.len() - 2]);

        let prod: u128 = inst.iter().map(|v| v.2).product();
        inv = Vec::new();
        cnt = Vec::new();

        for (items, _, _, _, _) in inst.iter() {
            inv.push(items.clone());
            cnt.push(0);
        }

        for _ in 0..10000 {
            for (idx, mon) in inst.iter().enumerate() {
                inv.push(Vec::new());
                let items = inv.swap_remove(idx);

                for item in items {
                    let mut w = item;

                    w = match mon.1 {
                        "old * 13" => w * 13,
                        "old + 4" => w + 4,
                        "old * 11" => w * 11,
                        "old + 8" => w + 8,
                        "old * old" => w * w,
                        "old + 5" => w + 5,
                        "old + 1" => w + 1,
                        "old + 3" => w + 3,
                        _ => unreachable!(),
                    };

                    w = w % prod;

                    let target = if w % mon.2 == 0 {
                        mon.3
                    } else {
                        mon.4
                    };

                    inv[target].push(w);
                    cnt[idx] = cnt[idx] + 1;
                }
            }
        }

        cnt.sort();
        print!("{:#?}\n", cnt[cnt.len() - 1] * cnt[cnt.len() - 2]);
    }

    /// Day12
    pub fn day12() {
        let input = std::io::read_to_string(std::io::stdin().lock()).unwrap();

        let mut inst = input.split("\n").filter(|v| v.len() > 0).map(
            |v| v.chars().map(|v| v as u32).collect::<Vec<u32>>(),
        ).collect::<Vec<Vec<u32>>>();

        let mut todo: BTreeMap<(usize, usize), u64> = BTreeMap::new();
        let mut done: BTreeMap<(usize, usize), u64> = BTreeMap::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        let width = inst[0].len();
        let height = inst.len();

        for (y, row) in inst.iter().enumerate() {
            for (x, entry) in row.iter().enumerate() {
                let d = if *entry == 'E' as u32 {
                    start = (x, y);
                    0
                } else {
                    if *entry == 'S' as u32 {
                        end = (x, y);
                    }
                    u64::MAX - 1
                };

                todo.insert((x, y), d);
            }
        }

        inst[start.1][start.0] = 'z' as u32;
        inst[end.1][end.0] = 'a' as u32;

        let mut cur_pos = start;
        loop {
            let cur_h = inst[cur_pos.1][cur_pos.0];
            let cur_v = *todo.get(&cur_pos).unwrap();

            if cur_pos.0 > 0 {
                let neigh_pos = (cur_pos.0 - 1, cur_pos.1);
                let neigh_h = inst[neigh_pos.1][neigh_pos.0];

                if cur_h <= neigh_h + 1 {
                    if let Some(neigh_v) = todo.get_mut(&neigh_pos) {
                        *neigh_v = u64::min(*neigh_v, cur_v + 1);
                    }
                }
            }
            if cur_pos.1 > 0 {
                let neigh_pos = (cur_pos.0, cur_pos.1 - 1);
                let neigh_h = inst[neigh_pos.1][neigh_pos.0];

                if cur_h <= neigh_h + 1 {
                    if let Some(neigh_v) = todo.get_mut(&neigh_pos) {
                        *neigh_v = u64::min(*neigh_v, cur_v + 1);
                    }
                }
            }
            if cur_pos.0 + 1 < width {
                let neigh_pos = (cur_pos.0 + 1, cur_pos.1);
                let neigh_h = inst[neigh_pos.1][neigh_pos.0];

                if cur_h <= neigh_h + 1 {
                    if let Some(neigh_v) = todo.get_mut(&neigh_pos) {
                        *neigh_v = u64::min(*neigh_v, cur_v + 1);
                    }
                }
            }
            if cur_pos.1 + 1 < height {
                let neigh_pos = (cur_pos.0, cur_pos.1 + 1);
                let neigh_h = inst[neigh_pos.1][neigh_pos.0];

                if cur_h <= neigh_h + 1 {
                    if let Some(neigh_v) = todo.get_mut(&neigh_pos) {
                        *neigh_v = u64::min(*neigh_v, cur_v + 1);
                    }
                }
            }

            done.insert(cur_pos, todo.remove(&cur_pos).unwrap());

            if todo.len() == 0 {
                break;
            }

            let mut next_pos = (0, 0);
            let mut next_v = u64::MAX;
            for (iter_pos, iter_v) in todo.iter() {
                if *iter_v < next_v {
                    next_pos = *iter_pos;
                    next_v = *iter_v;
                }
            }

            assert_ne!(next_v, u64::MAX);

            cur_pos = next_pos;
        }

        print!("END: {:#?}\n", *done.get(&end).unwrap());

        let mut min_v = u64::MAX;
        for (iter_pos, iter_v) in done.iter() {
            if inst[iter_pos.1][iter_pos.0] == 'a' as u32 {
                if *iter_v < min_v {
                    min_v = *iter_v;
                }
            }
        }

        print!("MIN: {:#?}\n", min_v);
    }

    /// Day12 Alternative
    pub fn day12_alt() {
        let input = std::io::read_to_string(std::io::stdin().lock()).unwrap();

        let mut inst = input.split("\n").filter(|v| v.len() > 0).map(
            |v| v.chars().map(|v| v as u32).collect::<Vec<u32>>(),
        ).collect::<Vec<Vec<u32>>>();

        let mut grid: Vec<Vec<u32>> = Vec::new();
        let width = inst[0].len();
        let height = inst.len();
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, row) in inst.iter().enumerate() {
            grid.push(Vec::new());
            for (x, entry) in row.iter().enumerate() {
                if *entry == 'S' as u32 {
                    start = (x, y);
                } else if *entry == 'E' as u32 {
                    end = (x, y);
                }
                grid[y].push(' ' as u32);
            }
        }

        inst[start.1][start.0] = 'a' as u32;
        inst[end.1][end.0] = 'z' as u32;

        for x in 0..width {
            for y in 0..height {
                let entry = inst[y][x];
                let mut dir: [bool; 4] = [false; 4];
                if x > 0 {
                    let v = inst[y][x - 1];
                    dir[0] = entry + 1 >= v;
                }
                if x + 1 < width {
                    let v = inst[y][x + 1];
                    dir[1] = entry + 1 >= v;
                }
                if y > 0 {
                    let v = inst[y - 1][x];
                    dir[2] = entry + 1 >= v;
                }
                if y + 1 < height {
                    let v = inst[y + 1][x];
                    dir[3] = entry + 1 >= v;
                }

                if dir[0] && dir[1] && dir[2] && dir[3] {
                    grid[y][x] = 0x254b;
                } else if dir[0] && dir[1] && dir[2] && !dir[3] {
                    grid[y][x] = 0x253b;
                } else if dir[0] && dir[1] && !dir[2] && dir[3] {
                    grid[y][x] = 0x2533;
                } else if dir[0] && dir[1] && !dir[2] && !dir[3] {
                    grid[y][x] = 0x2501;
                } else if dir[0] && !dir[1] && dir[2] && dir[3] {
                    grid[y][x] = 0x252b;
                } else if dir[0] && !dir[1] && dir[2] && !dir[3] {
                    grid[y][x] = 0x251b;
                } else if dir[0] && !dir[1] && !dir[2] && dir[3] {
                    grid[y][x] = 0x2513;
                } else if dir[0] && !dir[1] && !dir[2] && !dir[3] {
                    grid[y][x] = 0x2578;
                } else if !dir[0] && dir[1] && dir[2] && dir[3] {
                    grid[y][x] = 0x2523;
                } else if !dir[0] && dir[1] && dir[2] && !dir[3] {
                    grid[y][x] = 0x2517;
                } else if !dir[0] && dir[1] && !dir[2] && dir[3] {
                    grid[y][x] = 0x250f;
                } else if !dir[0] && dir[1] && !dir[2] && !dir[3] {
                    grid[y][x] = 0x257a;
                } else if !dir[0] && !dir[1] && dir[2] && dir[3] {
                    grid[y][x] = 0x2503;
                } else if !dir[0] && !dir[1] && dir[2] && !dir[3] {
                    grid[y][x] = 0x2579;
                } else if !dir[0] && !dir[1] && !dir[2] && dir[3] {
                    grid[y][x] = 0x257b;
                } else if !dir[0] && !dir[1] && !dir[2] && !dir[3] {
                    grid[y][x] = ' ' as u32;
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                let v = grid[y][x];
                if (x, y) == start {
                    print!("S");
                } else if (x, y) == end {
                    print!("E");
                } else if v == 0 {
                    print!(" ");
                } else {
                    print!("{}", char::from_u32(v).unwrap());
                }
            }
            print!("\n");
        }
    }
}
