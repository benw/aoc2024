fn main() {
    let input = include_str!("input.txt");
    //let input = "2333133121414131402";

    let mut blocks = spread(input);
    compact(&mut blocks);
    let checksum = checksum(&blocks);
    println!("Part 1: {}", checksum);
}

fn spread(input: &str) -> Vec<Option<u32>> {
    let mut it = input.chars();
    let mut blocks = vec![];
    let mut id = 0;
    loop {
        let Some(ch) = it.next() else {
            break;
        };
        let count = ch.to_digit(10).unwrap() as usize;
        blocks.extend(std::iter::repeat_n(Some(id), count));
        id += 1;

        let Some(ch) = it.next() else {
            break;
        };
        let count = ch.to_digit(10).unwrap() as usize;
        blocks.extend(std::iter::repeat_n(None, count));
    }
    blocks
}

fn compact(blocks: &mut Vec<Option<u32>>) {
    let mut i = 0;
    while i < blocks.len() {
        if blocks[i].is_none() {
            blocks[i] = blocks.pop().unwrap();
        } else {
            i += 1;
        }
    }
}

fn checksum(blocks: &[Option<u32>]) -> u64 {
    let mut total = 0;
    for (i, b) in blocks.iter().enumerate() {
        total += (i as u64) * (*b.as_ref().unwrap() as u64) ;
    }
    total
}
