use std::{fs, error, cmp};

fn main() -> std::result::Result<(), Box<dyn error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    // let input = fs::read_to_string("test.txt")?;

    let ass_red = 12;
    let ass_green = 13;
    let ass_blue = 14;

    let mut sum_valid_max = 0;
    let mut sum_of_powers = 0;

    for line in input.lines() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        
        let game_iter = &mut line[5..].split(":");
        let game_index = u32::from_str_radix(game_iter.next().unwrap(), 10)?;
        let subset_iter = game_iter.next().expect("No_subsets, whot").split(";");
        for subset in subset_iter {
            let color_iter = subset.split(",");
            for color in color_iter {
                let mut iter = color.split_whitespace();
                let val = u32::from_str_radix(iter.next().unwrap(), 10)?;
                let c = &iter.next().unwrap()[0..1];
                match c {
                    "r" => {
                        max_red = cmp::max(val, max_red);
                    }
                    "g" => {
                        max_green = cmp::max(val, max_green);
                    }
                    "b" => {
                        max_blue = cmp::max(val, max_blue);
                    }
                     _  => panic!("invalid input")
                }
            }
        }
        if ass_red >= max_red && ass_green >= max_green && ass_blue >= max_blue {
            sum_valid_max += game_index;
        }

        sum_of_powers += max_red * max_green * max_blue;

        // println!("{},{},{}", max_red, max_green, max_blue);
        
        
    }

    println!("{sum_valid_max}");
    println!("{sum_of_powers}");
    Ok(())
}
