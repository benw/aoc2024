use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    let doc = parser::document(input).unwrap();
    println!("{:#?}", doc);

    let mut by_a: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in &doc.rules {
        by_a.entry(rule.a).or_default().insert(rule.b);
    }

    let mut total = 0;
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
            total += middle(update);
        }
    }
    println!("Part 1: {}", total);
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
