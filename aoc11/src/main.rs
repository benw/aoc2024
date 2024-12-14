use std::collections::HashMap;

fn main() {
    let input = [965842, 9159, 3372473, 311, 0, 6, 86213, 48];
    //let input = [125, 17];

    let mut memo = HashMap::new();
    println!("Part 1: {}", stones(&input, 25, &mut memo));
    println!("Part 2: {}", stones(&input, 75, &mut memo));
}

fn stones(input: &[u64], blinks: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    let mut count = 0;
    for &i in input {
        count += f(i, blinks, memo);
    }
    count
}

fn f(i: u64, blinks: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(res) = memo.get(&(i, blinks)) {
        return *res;
    }
    let res = if blinks == 0 {
        1
    } else if i == 0 {
        f(1, blinks - 1, memo)
    } else {
        let dig = digits(i);
        if dig % 2 == 0 {
            let (left, right) = split(i, dig / 2);
            f(left, blinks - 1, memo) + f(right, blinks - 1, memo)
        } else {
            f(i * 2024, blinks - 1, memo)
        }
    };
    memo.insert((i, blinks), res);
    res
}

fn digits(mut i: u64) -> u32 {
    let mut dig = 0;
    while i > 0 {
        dig += 1;
        i /= 10;
    }
    dig
}

fn split(i: u64, dig: u32) -> (u64, u64) {
    let mul = 10u64.pow(dig);
    let right = i % mul;
    let left = i / mul;
    (left, right)
}
