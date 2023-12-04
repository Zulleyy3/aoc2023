use std::fs;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>>{
    let input = fs::read_to_string("input.txt")?; 
    // let input = fs::read_to_string("test.txt")?; 
    //
    let mut sum: usize = 0;
    let mut counts: Vec<u32> = input.lines().map(|_| 1u32).collect();
    for (i, line) in input.lines().enumerate(){
        let mut input =  line.split(":").nth(1).unwrap().split("|");
        let winning: Vec<u32> = input.next().unwrap().split_whitespace().map(|s| u32::from_str_radix(s, 10).unwrap()).collect();
        let ours: Vec<u32> = input.next().unwrap().split_whitespace().map(|s| u32::from_str_radix(s, 10).unwrap()).collect();
        let count_wins: u32 = ours.iter().filter(|v| winning.contains(v)).count().try_into().unwrap();
        if count_wins != 0 {
            sum += 2usize.pow(count_wins-1);
        }
        let count = counts[i];
        for elem in &mut counts[i+1..i+1+(count_wins as usize)].iter_mut(){
            *elem += count;
        }

    }

    let total_scratch: u32 = counts.iter().sum();
    println!("Cards in my collection {:?}", counts);

    println!("{}", sum );
    println!("{}", total_scratch);
    Ok(())
}
