fn main() {
    let input = [965842, 9159, 3372473, 311, 0, 6, 86213, 48];
    //let input = [125, 17];

    let mut v = Vec::from(&input);
    for i in 0..25 {
        let out = blink(&v);
        v = out;
        //println!("{}: {:?}", i, v);
        println!("{}: {}", i, v.len());
    }
    println!("Part 1: {}", v.len());
}

fn blink(input: &[u64]) -> Vec<u64> {
    let mut out = vec![];
    for &i in input {
        if i == 0 {
            out.push(1);
        } else {
            let dig = digits(i);
            if dig % 2 == 0 {
                let (left, right) = split(i, dig / 2);
                out.push(left);
                out.push(right);
            } else {
                out.push(i * 2024);
            }
        }
    }
    out
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
