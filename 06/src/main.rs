use std::fs;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    // part 1
    let mut lines = input.lines();
    let race_time = lines.next().unwrap()[5..].split_ascii_whitespace().map(|s| u64::from_str_radix(s, 10).unwrap());
    let distances = lines.next().unwrap()[9..].split_ascii_whitespace().map(|s| u64::from_str_radix(s,10).unwrap());
    let result: usize = race_time
        .zip(distances)
        .map(|(t, d)|{
            (1..t)
            .map(|hold| hold * (t-hold))
            .filter(|distance| distance > &d)
            .count()
        })
        .product();
        

    println!("Part 1: {}", result);
    
    let mut lines = input.lines();
    let race_time = lines.next().unwrap()[5..].replace(" ","");
    let race_time = u64::from_str_radix(&race_time, 10).unwrap();
    let distances = lines.next().unwrap()[9..].replace(" ","");
    let distances = u64::from_str_radix(&distances,10).unwrap();
    let result: usize =
            (1..race_time)
            .map(|hold| hold * (race_time-hold))
            .filter(|distance| distance > &distances)
            .count();
        

    println!("Part 2: {}", result);
    Ok(())
}
