fn main() {
    let input = include_str!("input.txt");
    
    let mut safe = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line.split_ascii_whitespace().map(|word| word.parse().unwrap()).collect();
        if is_safe(levels.into_iter()) {
            safe += 1;
        }
    }
    println!("Part 1: {}", safe);

    let mut safe = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line.split_ascii_whitespace().map(|word| word.parse().unwrap()).collect();
        for i in 0..levels.len() {
            let mut l = levels.clone();
            l.remove(i);
            if is_safe(l.into_iter()) {
                safe += 1;
                break;
            }
        }
    }
    println!("Part 2: {}", safe);
}

fn is_safe(mut levels: impl Iterator<Item=i32>) -> bool {
    let mut safe_increasing = true;
    let mut safe_decreasing = true;
    let mut a: i32 = levels.next().unwrap();
    for b in levels {
        if safe_decreasing && (b >= a || b < a - 3) {
            safe_decreasing = false;
        }
        if safe_increasing && (b <= a || b > a + 3) {
            safe_increasing = false;
        }
        a = b;
    }
    safe_increasing || safe_decreasing
}
