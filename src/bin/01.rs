use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        struct Cursor {
            acc: i32,
            zeros: usize,
        }

        let cursor = reader.lines().flatten().fold(Cursor { acc: 50, zeros: 0 }, |cursor, line| {
            let dir_str: &str = &line[0..1];
            let mul: i32 = match dir_str {
                "L" => -1,
                "R" => 1,
                _ => panic!("Unknown direction string: {}", dir_str)
            };

            let num_str: &str = &line[1..];
            let num: i32 = num_str.parse().unwrap();

            let val = mul * num;
            let acc = (cursor.acc + val) % 100;

            let zeros = match acc {
                0 => cursor.zeros + 1,
                _ => cursor.zeros,
            };

            Cursor { acc: acc, zeros: zeros }
        });
        Ok(cursor.zeros)
    }

    // Test
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
