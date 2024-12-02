use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn solve_a() {
    let file = File::open("src/year24/day2.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut count = 0;
    
    'outer: for line in lines.flatten() {
        let mut nums = line.split(' ').map(|s| {
            s.parse::<i8>().unwrap()
        });

        let mut prev = nums.next().unwrap();
        let mut is_increasing: Option<bool> = None;
        
        for num in nums {
            let dif = num - prev;
            
            if dif > 0 && dif <= 3 {
                match is_increasing {
                    None => is_increasing = Some(true),
                    Some(is_increasing) => if !is_increasing { continue 'outer; }
                }
                
            } else if dif < 0 && dif >= -3 {
                match is_increasing {
                    None => is_increasing = Some(false),
                    Some(is_increasing) => if is_increasing { continue 'outer; }
                }
                
            } else {
                continue 'outer;
            }
            
            prev = num;
        }
        
        count += 1;
    }
    
    println!("Result {count}");
}

pub fn solve_b() {
    
}