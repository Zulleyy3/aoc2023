use std::cmp::Ordering;
use std::rc::Rc;
use std::{error, fs, iter::Peekable, str::Lines};
#[derive(Eq, PartialEq, Clone, Copy)]
struct Mapping {
    dest_start: u64,
    source_start: u64,
    length: u64,
}


impl Ord for Mapping {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source_start.cmp(&other.source_start)
    }
}
impl PartialOrd for Mapping {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn push_mappings(list: &mut Vec<Mapping>, lines: &mut Peekable<Lines>) {
    while let Some(line) = lines.next_if(|l| !l.is_empty()) {
        println!("{}", line);
        let vals: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|s| u64::from_str_radix(s, 10).unwrap())
            .collect();
        list.push(Mapping {
            dest_start: vals[0],
            source_start: vals[1],
            length: vals[2],
        })
    }
    list.sort();
}

fn resolve_mapping(val: u64, mapper: &Vec<Mapping>) -> u64 {
    for mp in mapper {
        if val >= mp.source_start && val < (mp.source_start + mp.length) {
            return mp.dest_start + val - mp.source_start;
        }
    }
    return val;
}

fn resolve_mapping_range(val: &(u64, u64), mapper: &Vec<Mapping>) -> Vec<(u64, u64)> {
    let mut lower = val.0;
    let mut len = val.1;
    let mut list: Vec<(u64, u64)> = Vec::new();
    for mp in mapper {
        if lower + len > mp.source_start && mp.source_start + mp.length > lower {
            println!("{} {} map {} {}", lower, len, mp.source_start, mp.length);
            if lower < mp.source_start {
                let used_up = mp.source_start - lower;
                list.push((lower, used_up));
                lower = mp.source_start;
                len -= used_up;
            }
            if lower - mp.source_start + len <= mp.length {
                list.push((mp.dest_start + (lower - mp.source_start), len));
                len = 0;
            } else {
                let used_up = mp.length - (lower - mp.source_start);
                list.push((mp.dest_start + (lower - mp.source_start), used_up));
                lower += used_up;
                len -= used_up;
            }
        }
    }
    if len > 0 {
        list.push((lower, len));
    }
    return list;
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    // let input = fs::read_to_string("test.txt")?;
    let mut lines = input.lines().peekable();
    let seeds: Vec<u64> = lines.next().unwrap()[6..]
        .split_ascii_whitespace()
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();
    let seeds2: Vec<(u64, u64)> = seeds
        .iter()
        .step_by(2)
        .zip((seeds.iter().skip(1)).step_by(2))
        .map(|(i, len)| (i.clone(), len.clone()))
        .collect();

    let mut seed_iter:  Box<dyn Iterator<Item = u64>> = Box::new(seeds.into_iter()); 
    let mut seed2_iter:  Box<dyn Iterator<Item = (u64,u64)>> = Box::new(seeds2.into_iter()); 
    let mut mappings: Vec<Rc<Vec<Mapping>>> = Vec::new();
    while let Some(_) = lines.next() {
        lines.next(); 
   
        let mut list = Vec::new();
        push_mappings(&mut list, &mut lines);
        let clone = Rc::new(list);
        let clone1 = Rc::clone(&clone);
        mappings.push(Rc::clone(&clone));
        
        seed_iter = Box::new(seed_iter.map(move |s| resolve_mapping(s.clone(), &clone)));
        seed2_iter = Box::new(seed2_iter.flat_map(move |s| resolve_mapping_range(&s, &clone1)));
    }
    let smallest_loc = seed_iter.min().unwrap();
    let smallest_loc_2 = seed2_iter.map(|(s,_)| s).min().unwrap();


    println!("Part 1 {}", smallest_loc);
    println!("Part 2 {}", smallest_loc_2);

    Ok(())
}
