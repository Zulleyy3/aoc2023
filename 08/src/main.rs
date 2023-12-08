use std::{error::Error, fs, collections::HashMap, char};

fn lcm(a: &usize, b: &usize) -> usize {
    let greatestcd = gcd(&a, &b);
    return a*b /  greatestcd;
}

fn gcd(a: &usize, b: &usize) -> usize {
    let mut u = a.clone();
    let mut v = b.clone();
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }
    
    let gcd_exponent_on_two = (u|v).trailing_zeros();
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();
    
    while u != v {
        if u < v {
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    return u << gcd_exponent_on_two;

}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    // let input = fs::read_to_string("test.txt")?;
    let mut input_iter = input.lines();
    let path = input_iter.next().unwrap();
    input_iter.next();
    let mut map: HashMap<(&str,char), &str> = HashMap::new();

    let mut beam_states: Vec<String> = Vec::new();

    let mut count = 0;
    for line in input_iter {
        // XGS = (FDM, XCS)
        let state = &line[0..3];
        map.insert((state, 'L'), &line[7..10]);
        map.insert((state, 'R'), &line[12..15]);
        if &line[2..3] == "A" {
            beam_states.push(state.to_string());
        }
        count += 1;
    }
    let mut last_state = "AAA";

    for (i, step) in path.chars().cycle().enumerate() {
        print!("Step #{}: {}", i, last_state);
        if last_state == "ZZZ" {
            break;
        }
        let next_state = map.get(&(last_state, step)).unwrap();
        println!(" -{}-> {}", step, next_state);
        last_state = next_state;
    }        
    println!("");

    // This is not fully necessary BUT it let me visually verify that I can just use lcm() to find
    // the correct number
    let mut zindex: Vec<usize> = Vec::with_capacity(beam_states.capacity());

    for val in beam_states.iter() {
        let mut visited: Vec<String> = Vec::with_capacity(count);
        let mut search = String::from(val);
        'inner: for (search_index, step) in path.chars().cycle().enumerate() {
            let next_search = map.get(&(&search, step)).unwrap().to_string();
            search.push(step);
            search.push_str(&(search_index % path.len()).to_string());
            if let Some((i, _)) = visited.iter().enumerate().find(|(_, s)| search == **s) {
                println!("Cycle Start Index {} -> Cycle End Index {}", i, search_index);
                break 'inner;
            }
            visited.push(search);
            search = next_search;
        }
        for (i, s) in visited.iter().enumerate().filter(|(_, s)| s.contains("Z"))  {
            println!("Z Position: {}, Z Word {}" , i, s);
            zindex.push(i);
        }
    }

    

    let ret = zindex.iter().fold(1usize, |a, b| lcm(&a,&b));
    println!("{}", ret);
    Ok(())
}
