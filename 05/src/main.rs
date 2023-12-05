use std::cmp::Ordering;
use std::{error, fs, iter::Peekable, str::Lines};
#[derive(Eq, PartialEq)]
struct mapping {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl Ord for mapping {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source_start.cmp(&other.source_start)
    }
}
impl PartialOrd for mapping {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn push_mappings(list: &mut Vec<mapping>, lines: &mut Peekable<Lines>) {
    while let Some(line) = lines.next_if(|l| !l.is_empty()) {
        println!("{}", line);
        let vals: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|s| u64::from_str_radix(s, 10).unwrap())
            .collect();
        list.push(mapping {
            dest_start: vals[0],
            source_start: vals[1],
            length: vals[2],
        })
    }
    list.sort();
}

fn resolve_mapping(val: u64, mapper: &Vec<mapping>) -> u64 {
    for mp in mapper {
        if val >= mp.source_start && val < (mp.source_start + mp.length) {
            return mp.dest_start + val - mp.source_start;
        }
    }
    return val;
}

fn resolve_mapping_range(val: &(u64, u64), mapper: &Vec<mapping>) -> Vec<(u64, u64)> {
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
    lines.next();
    lines.next();
    let mut seed_2_soil: Vec<mapping> = Vec::new();
    push_mappings(&mut seed_2_soil, &mut lines);
    lines.next();
    lines.next();
    let mut soil_2_fert: Vec<mapping> = Vec::new();
    push_mappings(&mut soil_2_fert, &mut lines);

    lines.next();
    lines.next();
    let mut fert_2_water: Vec<mapping> = Vec::new();
    push_mappings(&mut fert_2_water, &mut lines);

    lines.next();
    lines.next();
    let mut water_2_light: Vec<mapping> = Vec::new();
    push_mappings(&mut water_2_light, &mut lines);

    lines.next();
    lines.next();
    let mut light_2_temp: Vec<mapping> = Vec::new();
    push_mappings(&mut light_2_temp, &mut lines);

    lines.next();
    lines.next();
    let mut temp_2_humid: Vec<mapping> = Vec::new();
    push_mappings(&mut temp_2_humid, &mut lines);

    lines.next();
    lines.next();
    let mut humid_2_loc: Vec<mapping> = Vec::new();
    push_mappings(&mut humid_2_loc, &mut lines);
    let smallest_loc = seeds
        .iter()
        .map(|s| resolve_mapping(s.clone(), &seed_2_soil))
        .map(|s| resolve_mapping(s.clone(), &soil_2_fert))
        .map(|s| resolve_mapping(s.clone(), &fert_2_water))
        .map(|s| resolve_mapping(s.clone(), &water_2_light))
        .map(|s| resolve_mapping(s.clone(), &light_2_temp))
        .map(|s| resolve_mapping(s.clone(), &temp_2_humid))
        .map(|s| resolve_mapping(s.clone(), &humid_2_loc))
        .min()
        .unwrap();
    let smallest_loc_2 = seeds2
        .iter()
        .flat_map(|s| resolve_mapping_range(&s, &seed_2_soil))
        .flat_map(|s| resolve_mapping_range(&s, &soil_2_fert))
        .flat_map(|s| resolve_mapping_range(&s, &fert_2_water))
        .flat_map(|s| resolve_mapping_range(&s, &water_2_light))
        .flat_map(|s| resolve_mapping_range(&s, &light_2_temp))
        .flat_map(|s| resolve_mapping_range(&s, &temp_2_humid))
        .flat_map(|s| resolve_mapping_range(&s, &humid_2_loc))
        .map(|(s,_)| s).min().unwrap();
    println!("Part 1 {}", smallest_loc);
    println!("Part 2 {}", smallest_loc_2);

    Ok(())
}
