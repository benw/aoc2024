fn main() {
    let input = include_str!("input.txt");
    //let input = "2333133121414131402";

    let (mut blocks, files) = spread(input);
    let mut blocks2 = blocks.clone();
    compact(&mut blocks);
    let sum = checksum(&blocks);
    println!("Part 1: {}", sum);

    compact2(&mut blocks2, &files);
    let sum = checksum(&blocks2);
    println!("Part 2: {}", sum);
}

#[derive(Debug)]
struct File {
    id: u32,
    index: usize,
    len: usize,
}

// (blocks = Vec<Option<id>>, files = Vec<File> in id order)
fn spread(input: &str) -> (Vec<Option<u32>>, Vec<File>) {
    let mut it = input.chars();
    let mut blocks = vec![];
    let mut files = vec![];
    let mut id = 0;
    loop {
        let Some(ch) = it.next() else {
            break;
        };
        let len = ch.to_digit(10).unwrap() as usize;
        files.push(File { id, index: blocks.len(), len });
        blocks.extend(std::iter::repeat_n(Some(id), len));
        id += 1;

        let Some(ch) = it.next() else {
            break;
        };
        let len = ch.to_digit(10).unwrap() as usize;
        blocks.extend(std::iter::repeat_n(None, len));
    }
    (blocks, files)
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

fn compact2(blocks: &mut Vec<Option<u32>>, files: &[File]) {
    for file in files.iter().rev() {
        if let Some(index) = find_free(&blocks[..file.index], file.len) {
            blocks[index..index+file.len].fill(Some(file.id));
            blocks[file.index..file.index+file.len].fill(None);
            //print_blocks(blocks);
        }

    }
}

fn find_free(blocks: &[Option<u32>], len: usize) -> Option<usize> {
    for i in 0..blocks.len() {
        if i+len <= blocks.len() && blocks[i..i+len].iter().all(|block| block.is_none()) {
            return Some(i);
        }
    }
    None
}

fn checksum(blocks: &[Option<u32>]) -> u64 {
    let mut total = 0;
    for (i, b) in blocks.iter().enumerate() {
        total += (i as u64) * (*b.as_ref().unwrap_or(&0) as u64) ;
    }
    total
}

fn print_blocks(blocks: &[Option<u32>]) {
    for block in blocks {
        match block {
            Some(id) => print!("{}", id),
            None => print!("."),
        }
    }
    println!("");
}
