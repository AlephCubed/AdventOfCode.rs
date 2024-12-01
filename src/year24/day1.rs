use std::fs;

pub fn solve_a() {
    let contents = fs::read_to_string("src/year24/day1.txt")
        .expect("file present");
    
    let (mut a, mut b): (Vec<&str>, Vec<&str>) = contents.split('\n').map(|s| {
        let mut split = s.splitn(2, ' ');
        (split.next().unwrap(), split.next().unwrap())
    }).unzip();
    
    a.sort();
    b.sort();
    
    let result: u32 = a.iter().zip(b).map(|(a, b)| {
        u32::abs_diff(a.parse().unwrap(), b.strip_prefix("  ").unwrap().parse().unwrap())
    }).sum();
    
    println!("The result is {result}.");
}

pub fn solve_b() {
    let contents = fs::read_to_string("src/year24/day1.txt")
        .expect("file present");

    let (mut a, mut b): (Vec<&str>, Vec<&str>) = contents.split('\n').map(|s| {
        let mut split = s.splitn(2, ' ');
        (split.next().unwrap(), split.next().unwrap())
    }).unzip();

    a.sort();
    b.sort();

    let mut result = 0;
    
    let b: Vec<u32> = b.iter().map(|s| {
        s.strip_prefix("  ").unwrap().parse().unwrap()
    }).collect();
    
    for a in a.iter().map(|s| s.parse::<u32>().unwrap()) {
        let Ok(index) = b.binary_search(&a) else { continue };
        
        let mut count = 1;
        
        let mut index_forward = index;
        while let Some(i) = b.get(index_forward + 1) {
            if *i != a { break; }
            count += 1;
            index_forward += 1;
        }

        let mut index_backward = index;
        while let Some(i) = b.get(index_backward - 1) {
            if *i != a { break }
            count += 1;
            index_backward -= 1;
        }
        
        result += a * count;
    }

    println!("The result is {result}.");
}