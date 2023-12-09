use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    // let input = fs::read_to_string("test.txt")?;
    let input = fs::read_to_string("input.txt")?;
    let mut sum: i64 = 0;
    let mut sum2: i64 = 0;
    for line in input.lines() {
        let mut nums: Vec<i64> = line.split_whitespace().map(|num| i64::from_str_radix(num, 10).unwrap()).collect();
        println!("{:?}", nums);
        // nums.pop();
        let mut numbers:Vec<Vec<i64>> = Vec::from([nums]);
        loop {
            let nums = numbers.last().unwrap(); 
            if let None = nums.iter().find(|num| **num != 0) {
                break;
            }
            let mut tmp: Vec<i64> = Vec::new();
            for i in 0..nums.len()-1 {
                tmp.push(nums[i+1] - nums[i]);
            }
            println!("{:?}",tmp);
            numbers.push(tmp);
        }
        numbers.last_mut().unwrap().insert(0,0);
        numbers.last_mut().unwrap().push(0);
        println!("{:?}", numbers.last().unwrap());

        for i in (1..numbers.len()).rev() {
            let prev = &numbers[i];
            let next = prev.last().unwrap() + numbers[i-1].last().unwrap();
            let next_0 = numbers[i-1][0] - prev[0];
            numbers[i-1].insert(0, next_0);
            numbers[i-1].push(next);
            println!("{:?}",numbers[i-1]);
        }
        sum += numbers[0].last().unwrap();
        sum2 += numbers[0][0];
        println!("");
    }

    println!("{}", sum);
    println!("{}", sum2);
    Ok(())
}
