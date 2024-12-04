use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let input = include_str!("input.txt");

    let mut total = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        let a: u32 = a.parse().unwrap();
        let b: u32 = b.parse().unwrap();
        total += a * b;
    }
    println!("Part 1: {}", total);

    let tokens = parser::document(input).unwrap();
    let mut total = 0;
    let mut enabled = true;
    for token in tokens {
        match token {
            Token::Do => enabled = true,
            Token::DoNot => enabled = false,
            Token::Mul(a, b) => if enabled { total += a * b },
            Token::Ignore => ()
        }
    }
    println!("Part 2: {}", total);
}

#[derive(Debug)]
enum Token {
    Do,
    DoNot,
    Mul(u32, u32),
    Ignore,
}


peg::parser!{
    grammar parser() for str {

        rule num() -> u32
            = n:$(['0'..='9']*<1,3>) { n.parse().unwrap() }

        rule token() -> Token
            = "do()" { Token::Do }
            / "don't()" { Token::DoNot }
            / "mul(" a:num() "," b:num() ")" { Token::Mul(a, b) }
            / [_] { Token::Ignore }

        pub rule document() -> Vec<Token>
            = token()*
    }
}
