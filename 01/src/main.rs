use std::char;
use std::error;
use std::fs;

fn comp_from_index(index: usize, source: &str, cmp: &str) -> bool {
    if index + cmp.len() > source.len() {
        return false;
    }
    return &source[index..(index + cmp.len())] == cmp;
}

fn looper<'a, I: Iterator<Item = (usize,char)>> ( iter: &mut I, source: &'a str) -> &'a str {
    loop {
            let (i, c) = match iter.next() {
                None => return "",
                Some(v) => v,
            };
            match c {
                '0'..='9' => return &source[i..i+1],
                'o' => {
                    if comp_from_index(i, source, "one") {
                        return "1"
                    }
                }
                't' => {
                    if comp_from_index(i, source, "two") {
                        return "2";
                    } else if comp_from_index(i, source, "three") {
                        return "3";
                    }
                }
                'f' => {
                    if comp_from_index(i, source, "four") {
                        return "4";
                    } else if comp_from_index(i, source, "five") {
                        return "5";
                    }
                }
                's' => {
                    if comp_from_index(i, source, "six") {
                        return "6"
                    } else if comp_from_index(i, source, "seven") {
                        return "7"
                    }
                }
                'e' => {
                    if comp_from_index(i, source, "eight") {
                        return "8"
                    }
                }
                'n' => {
                    if comp_from_index(i, source, "nine") {
                        return "9"
                    }
                }
                _ => continue,
            };
        };
}

fn main() -> std::result::Result<(), Box<dyn error::Error>> {
    let body: String = fs::read_to_string("input.txt")?;
    // let body: String = fs::read_to_string("test.txt")?;
    let mut sum_1: i32 = 0;
    let mut sum_2: i32 = 0;
    for line in body.lines() {
        // // Part 1
        let part1 = line.chars().skip_while( |c| !c.is_ascii_digit()).next().unwrap();
        let part2 = line.chars().rev().skip_while( |c| !c.is_ascii_digit()).next().unwrap();
        let string = format!("{part1}{part2}");
        let val = i32::from_str_radix(&string, 10)?;
        sum_1 += val;
        
        // // Part 2
        let part1 = looper(&mut line.char_indices(), line);
        let part2 = looper(&mut line.char_indices().rev(), line);
        let string = format!("{part1}{part2}");
        let val = i32::from_str_radix(&string, 10)?;
        sum_2 += val;
    }
    println!("Part 1: {}", sum_1);
    println!("Part 2: {}", sum_2);

    Ok(())
}
