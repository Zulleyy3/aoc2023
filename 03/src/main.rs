use std::char;
use std::fs;
use std::error;
use std::cmp;

fn check_symbol(prev: Option<&str>, next: Option<&str>, cur: &str, begin: usize, end: usize) -> u32 {
    let begin_check =  if begin.wrapping_sub(1) < begin {begin.wrapping_sub(1)} else {0};
    // Upper wrap is very unlikely with this task so we don't bother, as we are bounded by string
    // length anyway
    let end_check =  if end + 1 < cur.len() {end + 1} else {end};
    // println!("{},{} -- {} {}: {}", begin, end, begin_check, end_check , cur);
    
    let mut symbol = false;

    symbol |= begin.wrapping_sub(1) < begin && &cur[begin_check..begin] != "." ; 
    symbol |= end + 1 < cur.len() && &cur[end..end_check] != "." ; 
    if let Some(prev_line) = prev {
        symbol |= &prev_line[begin_check..end_check].contains(|c| c != '.');
    }
    if let Some(next_line) = next {
        symbol |= &next_line[begin_check..end_check].contains(|c| c != '.');
    }

    if symbol {
        u32::from_str_radix(&cur[begin..end], 10).unwrap()
    } else {0}

}


fn main() -> Result<(), Box<dyn error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut prev_line: Option<&str>= None;
    
    let mut line_iter = input.lines().peekable();

    let mut sum = 0;
    
    while let Some(line) = line_iter.next() {
        let mut char_iter = line.char_indices().peekable();
        let next_line: Option<&&str> = line_iter.peek();
        while let Some((i, c)) = char_iter.next() {
            if c.is_ascii_digit() {
                while char_iter.next_if(|(_, chr) | chr.is_ascii_digit()).is_some() {}
                
                let to_add = if let Some((end_i,_)) = char_iter.next() {
                    check_symbol(prev_line, next_line.copied(), line, i, end_i)
                } else {
                    check_symbol(prev_line, next_line.copied(), line, i, line.len())
                };

                sum += to_add;
            }
        }
        prev_line = Some(line);
    }
    
    println!("{}",sum);

    Ok(())
}
