fn main() {
    let input = include_str!("input.txt");

    let rows = parser::document(input).unwrap();
    
    let mut total = 0;
    for row in &rows {
        if row.solves() {
            total += row.value;
        }
    }
    println!("Part 1: {}", total);

    let mut total = 0;
    for row in &rows {
        if row.solves2() {
            total += row.value;
        }
    }
    println!("Part 2: {}", total);
}

#[derive(Debug)]
struct Row {
    value: u64,
    terms: Vec<u64>,
}

impl Row {
    fn solves(&self) -> bool {
        self.inner(self.terms[0], &self.terms[1..])
    }

    fn inner(&self, acc: u64, rhs: &[u64]) -> bool {
        if rhs.len() == 0 {
            acc == self.value
        } else {
            self.inner(acc + rhs[0], &rhs[1..]) || self.inner(acc * rhs[0], &rhs[1..])
        }
    }

    fn solves2(&self) -> bool {
        self.inner2(self.terms[0], &self.terms[1..])
    }

    fn inner2(&self, acc: u64, rhs: &[u64]) -> bool {
        if rhs.len() == 0 {
            acc == self.value
        } else {
            self.inner2(acc + rhs[0], &rhs[1..]) || self.inner2(acc * rhs[0], &rhs[1..]) || self.inner2(concat(acc, rhs[0]), &rhs[1..])
        }
    }
}

fn concat(mut a: u64, b: u64) -> u64 {
    let mut bb = b;
    while bb > 0 {
        a *= 10;
        bb /= 10;
    }
    a + b
}

#[test]
fn test_concat() {
    assert_eq!(12345, concat(123, 45));
    assert_eq!(486, concat(48, 6));
}

peg::parser!{
    grammar parser() for str {

        rule num() -> u64
            = n:$(['0'..='9']+) { n.parse().unwrap() }
        
        rule terms() -> Vec<u64>
            = num()++" "

        rule row() -> Row
            = value:num() ": " terms:terms() "\n" { Row { value, terms } }

        pub rule document() -> Vec<Row>
            = row()*
    }
}
