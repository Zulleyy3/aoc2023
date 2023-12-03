use std::collections::HashMap;
use std::error;
use std::fs;

fn check_symbol(
    prev: Option<(usize, &str)>,
    next: Option<(usize, &str)>,
    line_num: usize,
    cur: &str,
    begin: usize,
    end: usize,
    map: &mut HashMap<(usize, usize), Vec<u32>>,
) -> u32 {
    let begin_check = if begin.wrapping_sub(1) < begin {
        begin.wrapping_sub(1)
    } else {
        0
    };
    // Upper wrap is very unlikely with this task so we don't bother, as we are bounded by string
    // length anyway
    let end_check = if end + 1 < cur.len() { end + 1 } else { end };

    let val = u32::from_str_radix(&cur[begin..end], 10).unwrap();
    let mut found = false;

    if begin.wrapping_sub(1) < begin {
        if &cur[begin_check..begin] != "." {
            if &cur[begin_check..begin] == "*" {
                append_or_create(map, line_num, begin_check, val);
            }
            found = true;
        }
    }
    if end + 1 < cur.len() {
        if &cur[end..end_check] != "." {
            if &cur[end..end_check] == "*" {
                append_or_create(map, line_num, end, val);
            }
            found = true;
        }
    }

    if let Some((prev_line_num, prev_line)) = prev {
        for (i, c) in (&prev_line[begin_check..end_check]).char_indices() {
            if c != '.' {
                if c == '*' {
                    append_or_create(map, prev_line_num, begin_check + i, val);
                }
                found = true;
            }
        }
    }

    if let Some((next_line_num, next_line)) = next {
        for (i, c) in (&next_line[begin_check..end_check]).char_indices() {
            if c != '.' {
                if c == '*' {
                    append_or_create(map, next_line_num, begin_check + i, val);
                }
                found = true;
            }
        }
    }
    if found {
        val
    } else {
        0
    }
}

fn append_or_create(
    map: &mut HashMap<(usize, usize), Vec<u32>>,
    line: usize,
    index: usize,
    val: u32,
) {
    if let Some(list) = &mut map.get_mut(&(line, index)) {
        list.push(val);
    } else {
        map.insert((line, index), Vec::from([val]));
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    // let input = fs::read_to_string("test.txt")?;
    let mut prev_line: Option<(usize, &str)> = None;

    let mut line_iter = input.lines().enumerate().peekable();
    let mut map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let mut sum = 0;

    while let Some((line_num, line)) = line_iter.next() {
        let mut char_iter = line.char_indices().peekable();
        let next_line: Option<&(usize, &str)> = line_iter.peek();
        while let Some((i, c)) = char_iter.next() {
            if c.is_ascii_digit() {
                while char_iter.next_if(|(_, chr)| chr.is_ascii_digit()).is_some() {}

                let to_add = if let Some((end_i, _)) = char_iter.next() {
                    check_symbol(
                        prev_line,
                        next_line.copied(),
                        line_num,
                        line,
                        i,
                        end_i,
                        &mut map,
                    )
                } else {
                    check_symbol(
                        prev_line,
                        next_line.copied(),
                        line_num,
                        line,
                        i,
                        line.len(),
                        &mut map,
                    )
                };

                sum += to_add;
            }
        }
        prev_line = Some((line_num, line));
    }
    let sum_gears: u32 = map
        .into_values()
        .filter(|l| l.len() == 2)
        .map(|l| l.iter().product::<u32>())
        .sum();

    println!("part 1: {}", sum);
    println!("part 2: {}", sum_gears);

    Ok(())
}
