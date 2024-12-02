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
    let file = File::open("src/year24/day2.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut count = 0;

    'outer: for line in lines.flatten() {
        println!("\nLine: {line}");
        let mut nums = line.split(' ').map(|s| {
            s.parse::<i8>().unwrap()
        });

        let mut a = nums.next().unwrap();
        let mut state = State::Equal;
        let mut used = false;
        let mut ignored = false;
        
        for (b, c) in nums.clone().zip(nums.clone().skip(1)) {
            let mut ig = false;
            println!("{a}, {b}, {c}, state: {:?}, used: {used}.", state);
            let state_a = get_state(a, b);
            let state_b = get_state(b, c);
            let state_c = get_state(a, c);
            println!("a: {:?}, b: {:?}, c: {:?}.", state_a, state_b, state_c);
            
            if state_a == state_b {
                if !state_a.is_err() { // All good.
                    println!("Good.");
                    a = b;
                    state = state_a;
                    
                // Three errors, unable to remove middle value (b). Unrecoverable.
                } else if state_c.is_err() || used { 
                    println!("Three Errors or used.\n");
                    continue 'outer; 
                    
                } else {
                    // Skip middle value (b).
                    println!("Skipping b: {b}.");
                    used = true;
                    state = state_c;
                }
            } else { // Try to recover.
                // We already used, or they are mismatched errors, we are done.
                if state_a.is_err() && state_b.is_err() {
                    println!("Mismatched errors.");
                    continue 'outer;
                }

                
                if state_a.is_err() {
                    // If first pass and error is (a), remove (a).
                    if state.is_err() {
                        if used { 
                            println!("Already used (1).\n"); 
                            continue 'outer; 
                        }
                        
                        println!("Skipping a: {a}.");
                        used = true;
                        a = b;
                        state = state_b;
                    } else if state_c.is_err() {
                        println!("(a) and (c) are errors.");
                        continue 'outer;
                    } else {
                        if used {
                            println!("Already used (1.5).\n");
                            continue 'outer;
                        }

                        println!("Ignoring - (b) error.");
                        ig = true;
                        a = b;
                        state = state_a;
                    }
                } else if state_b.is_err() {
                    if !state_c.is_err() { // Try removing (b).
                        if used {
                            println!("Already used (2).\n");
                            continue 'outer;
                        }

                        println!("Skipping b1.5: {b}.");
                        used = true;
                        state = state_c;
                    } else {
                        // (a) is ok, (b) is err, (c) is err.
                        println!("Ignoring - (b) (c) err.");
                        ig = true;
                        a = b;
                        state = state_a;
                    }

                // Neither are errors, but don't match.
                } else if !state_c.is_err() {
                    if state_c == state || state.is_err() {
                        if used {
                            println!("Already used (3).\n");
                            continue 'outer;
                        }
                        
                        println!("Skipping b2: {b}.");
                        used = true;
                        state = state_c;
                    } else if state_a == state {
                        if used {
                            println!("Already used (4).\n");
                            continue 'outer;
                        }
                        
                        println!("Ignoring - Deal with peak next iteration.");
                        ig = true;
                        a = b;
                    } else {
                        println!("Unable to remove peak.");
                        continue 'outer;
                    }
                } else {// Ignore. Catch on the next iteration.
                    if state.is_err() {// First iteration, can remove (a).
                        if used {
                            println!("Already used (5).\n");
                            continue 'outer;
                        }
                        
                        println!("Skipping a2: {a}.");
                        used = true;
                        a = b;
                    } else {
                        if used {
                            println!("Already used (6).\n");
                            continue 'outer;
                        }
                        
                        println!("Ignoring - Can't skip (a) or (b).");
                        ig = true;
                        a = b;
                        state = state_a;
                    }
                }
            }
            
            if ig && ignored {
                println!("Double ignored.");
                continue 'outer;
            }
            ignored = ig;
        }
        
        if ignored && used {
            let mut rev = nums.rev();
            let a = rev.next().unwrap();
            let b = rev.next().unwrap();
            let stat_a = get_state(a, b);
            
            if state != stat_a {
                println!("Ignored final with use, err.");
                continue 'outer;
            }
            println!("Ignored final with use, ok.");
        }

        println!("Passed!");
        count += 1;
    }

    println!("Result {count}");
}

#[derive(Eq, PartialEq, Debug)]
enum State {
    Equal,
    Increasing,
    Decreasing,
    OutOfBounds,
}

impl State {
    fn is_err(&self) -> bool {
        match self {
            State::Increasing | State::Decreasing => false,
            _ => true,
        }
    }
}

fn get_state(a: i8, b: i8) -> State {
    let dif = a - b;
    
    if dif == 0 {
        State::Equal
    } else if dif > 0 && dif <= 3 {
        State::Decreasing
    } else if dif < 0 && dif >= -3 {
        State::Increasing
    } else {
        State::OutOfBounds
    }
}