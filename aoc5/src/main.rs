use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    let doc = parser::document(input).unwrap();

    let mut by_a: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut by_b: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in &doc.rules {
        by_a.entry(rule.a).or_default().insert(rule.b);
        by_b.entry(rule.b).or_default().insert(rule.a);
    }

    let mut part1_total = 0;
    let mut part2_total = 0;
    for update in &doc.updates {
        let mut valid = true;
        'update: for (i, a) in update.iter().enumerate() {
            for b in &update[..i] {
                let Some(bset) = by_a.get(a) else {
                    continue;
                };
                if bset.contains(b) {
                    println!("Invalid due to {}|{} in {:?} - ", a, b, update);
                    valid = false;
                    break 'update;
                }
            }
        }
        if valid {
            println!("Valid: {:?}", update);
            part1_total += middle(update);
        } else {
            println!("Invalid: {:?}", update);
            let sorted = sort(update, &by_b);
            part2_total += middle(&sorted);
        }
    }
    println!("Part 1: {}", part1_total);
    println!("Part 2: {}", part2_total);
}

fn sort(update: &[u32], by_b: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let mut sorted = vec![];
    let mut stack = vec![];
    for &x in update {
        append_ordered(&mut sorted, x, by_b, &mut stack, update);
    }
    sorted
}

fn append_ordered(sorted: &mut Vec<u32>, x: u32, by_b: &HashMap<u32, HashSet<u32>>, stack: &mut Vec<u32>, update: &[u32]) {
    if sorted.contains(&x) || stack.contains(&x) || !update.contains(&x) {
        return;
    }
    stack.push(x);
    // aset is set of values that must appear before x
    if let Some(aset) = by_b.get(&x) {
        for &a in aset {
            append_ordered(sorted, a, by_b, stack, update);
        }
    }
    assert_eq!(x, stack.pop().unwrap());
    sorted.push(x);
}

#[derive(Debug)]
struct Rule {
    a: u32,
    b: u32,
}

#[derive(Debug)]
struct Doc {
    rules: Vec<Rule>,
    updates: Vec<Vec<u32>>,
}

fn middle(update: &[u32]) -> u32 {
    let mid = update.len() / 2;
    update[mid]
}

peg::parser!{
    grammar parser() for str {

        rule num() -> u32
            = n:$(['0'..='9']*<1,3>) { n.parse().unwrap() }

        rule rool() -> Rule
            = a:num() "|" b:num() "\n" { Rule { a, b } }
        
        rule update() -> Vec<u32>
            = n:num()++"," "\n" { n }

        pub rule document() -> Doc
            = rules:rool()* "\n" updates:update()* { Doc { rules, updates } }
    }
}
