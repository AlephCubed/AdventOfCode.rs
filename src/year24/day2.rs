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

    for line in lines.flatten() {
        println!("\nLine: {line}");
        let nums: Vec<i8> = line.split(' ').map(|s| {
            s.parse::<i8>().unwrap()
        }).collect();

        if check_line(nums) {
            count += 1;
        }
    }

    println!("Result {count}");
}

fn check_line(nums: Vec<i8>) -> bool {
    let mut nums = nums.iter();
    
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
                return false;

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
                return false;
            }


            if state_a.is_err() {
                // If first pass and error is (a), remove (a).
                if state.is_err() {
                    if used {
                        println!("Already used (1).\n");
                        return false;
                    }

                    println!("Skipping a: {a}.");
                    used = true;
                    a = b;
                    state = state_b;
                } else if state_c.is_err() {
                    println!("(a) and (c) are errors.");
                    return false;
                } else {
                    if used {
                        println!("Already used (1.5).\n");
                        return false;
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
                        return false;
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
                        return false;
                    }

                    println!("Skipping b2: {b}.");
                    used = true;
                    state = state_c;
                } else if state_a == state {
                    if used {
                        println!("Already used (4).\n");
                        return false;
                    }

                    println!("Ignoring - Deal with peak next iteration.");
                    ig = true;
                    a = b;
                } else {
                    println!("Unable to remove peak.");
                    return false;
                }
            } else {// Ignore. Catch on the next iteration.
                if state.is_err() {// First iteration, can remove (a).
                    if used {
                        println!("Already used (5).\n");
                        return false;
                    }

                    println!("Skipping a2: {a}.");
                    used = true;
                    a = b;
                } else {
                    if used {
                        println!("Already used (6).\n");
                        return false;
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
            return false;
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
            return false;
        }
        println!("Ignored final with use, ok.");
    }

    println!("Passed!");
    true
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

fn get_state(a: &i8, b: &i8) -> State {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example() {
        assert_eq!(check_line(vec![7, 6, 4, 2, 1]), true);
        assert_eq!(check_line(vec![1, 2, 7, 8, 9]), false);
        assert_eq!(check_line(vec![9, 7, 6, 2, 1]), false);
        assert_eq!(check_line(vec![1, 3, 2, 4, 5]), true);
        assert_eq!(check_line(vec![8, 6, 4, 4, 1]), true);
        assert_eq!(check_line(vec![1, 3, 6, 7, 9]), true);
    }
    
    //https://www.reddit.com/r/adventofcode/comments/1h4shdu/2024_day_2_part2_edge_case_finder/
    #[test]
    fn test_edge_cases() {
        assert_eq!(check_line(vec![48, 46, 47, 49, 51, 54, 56]), true);
        assert_eq!(check_line(vec![1, 1, 2, 3, 4, 5]), true);
        assert_eq!(check_line(vec![1, 2, 3, 4, 5, 5]), true);
        assert_eq!(check_line(vec![5, 1, 2, 3, 4, 5]), true);
        assert_eq!(check_line(vec![1, 4, 3, 2, 1]), true);
        assert_eq!(check_line(vec![1, 6, 7, 8, 9]), true);
        assert_eq!(check_line(vec![1, 2, 3, 4, 3]), true);
        assert_eq!(check_line(vec![9, 8, 7, 6, 7]), true);
        assert_eq!(check_line(vec![7, 10, 8, 10, 11]), true);
        assert_eq!(check_line(vec![29, 28, 27, 25, 26, 25, 22, 20]), true);
    }
    
    #[test]
    fn test_simple_one_offset() {
        assert_eq!(check_line(vec![1, 2, 3, 4]), true);
    }

    #[test]
    fn test_simple_two_offset() {
        assert_eq!(check_line(vec![1, 3, 5, 7]), true);
    }

    #[test]
    fn test_simple_three_offset() {
        assert_eq!(check_line(vec![1, 4, 7, 10]), true);
    }

    #[test]
    fn test_simple_down() {
        assert_eq!(check_line(vec![4, 3, 2, 1]), true);
    }
    
    #[test]
    fn test_err_jump() {
        assert_eq!(check_line(vec![1, 3, 7, 8]), false);
    }

    #[test]
    fn test_err_jump_down() {
        assert_eq!(check_line(vec![10, 7, 3, 1]), false);
    }

    #[test]
    fn test_err_dup_save() {
        assert_eq!(check_line(vec![1, 2, 2, 3]), true);
    }

    #[test]
    fn test_err_trip() {
        assert_eq!(check_line(vec![1, 2, 2, 2]), false);
    }
    
    #[test]
    fn test_err_peak_save_end() {
        assert_eq!(check_line(vec![1, 2, 4, 3]), true);
    }

    #[test]
    fn test_err_peak_dup() {
        assert_eq!(check_line(vec![1, 2, 3, 2, 3]), false);
    }

    #[test]
    fn test_err_peak_swap() {
        assert_eq!(check_line(vec![1, 2, 3, 1, 3]), false);
    }

    #[test]
    fn test_err_peak_swap_end_save() {
        assert_eq!(check_line(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn test_err_peak_swap_begin_save() {
        assert_eq!(check_line(vec![2, 3, 1, 0]), true);
    }

    #[test]
    fn test_err_jump_end_save() {
        assert_eq!(check_line(vec![1, 3, 6, 10]), true);
    }

    #[test]
    fn test_err_jump_begin_save() {
        assert_eq!(check_line(vec![1, 4, 7, 10]), true);
    }
    
    // -----------------=================-----------------

    #[test]
    fn test_err_double_jump_begin_end() {
        assert_eq!(check_line(vec![1, 5, 6, 10]), false);
    }

    #[test]
    fn test_err_double_dup() {
        assert_eq!(check_line(vec![1, 1, 2, 2]), false);
    }

    #[test]
    fn test_err_double_peak() {
        assert_eq!(check_line(vec![1, 3, 2, 3, 4, 2]), false);
    }

    #[test]
    fn test_err_double_peak_dup_end() {
        assert_eq!(check_line(vec![1, 3, 2, 3, 4, 3]), false);
    }

    #[test]
    fn test_err_jump_peak_save() {
        assert_eq!(check_line(vec![1, 4, 3]), true);
    }

    #[test]
    fn test_err_peak_save() {
        assert_eq!(check_line(vec![4, 5, 1]), true);
    }

    #[test]
    fn test_err_peak_jump() {
        assert_eq!(check_line(vec![4, 5, 0]), true);
    }

    #[test]
    fn test_err_jump_dup() {
        assert_eq!(check_line(vec![0, 4, 4]), false);
    }

    #[test]
    fn test_err_jump_gap_dup() {
        assert_eq!(check_line(vec![0, 4, 3, 3]), false);
    }

    #[test]
    fn test_err_peak_gap_dup() {
        assert_eq!(check_line(vec![0, 1, 4, 3, 4, 4]), false);
    }

    // -----------------=================-----------------
    
    #[test]
    fn test_case_dup_plus_peak() {
        assert_eq!(check_line(vec![8, 9, 11, 11, 13, 10]), false);
    }
    
    #[test]
    fn test_case_dup_plus_end_jump() {
        assert_eq!(check_line(vec![25, 27, 28, 28, 31, 35]), false);
    }
    
    #[test]
    fn test_case_dup_plus_2_jump() {
        assert_eq!(check_line(vec![71, 73, 73, 76, 82]), false);
    }
    
    #[test]
    fn test_case_2_end_jump() {
        assert_eq!(check_line(vec![3, 5, 6, 8, 11, 12, 16, 19]), false);
    }
    #[test]
    fn test_case_jump_plus_dup_peak() {
        assert_eq!(check_line(vec![59, 62, 66, 69, 71, 69]), false);
    }
    
    #[test]
    fn test_case_jump_plus_dup() {
        assert_eq!(check_line(vec![19, 20, 24, 25, 26, 26]), false);
    }
    
    #[test]
    fn test_case_2_jump() {
        assert_eq!(check_line(vec![1, 2, 6, 8, 12]), false);
    }
    
    #[test]
    fn test_case_jump_plus_end_jump() {
        assert_eq!(check_line(vec![61, 62, 63, 66, 68, 69, 73, 78]), false);
    }
    
    #[test]
    fn test_case_large_jumps() {
        assert_eq!(check_line(vec![43, 46, 48, 55, 58, 59, 60, 63]), false);
    }
    
    #[test]
    fn test_case_large_jump_plus_peak() {
        assert_eq!(check_line(vec![35, 38, 40, 45, 46, 47, 44]), false);
    }
    
    #[test]
    fn test_case_end_jump_plus_dup() {
        assert_eq!(check_line(vec![84, 85, 86, 87, 94, 94]), false);
    }
    
    #[test]
    fn test_case_large_jump2() {
        assert_eq!(check_line(vec![80, 83, 90, 91, 93, 95, 99]), false);
    }
}