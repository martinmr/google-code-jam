use std::io;
use std::io::BufRead;
use std::io::Lines;
use std::io::StdinLock;

fn parse_vector(line: &String) -> Vec<i64> {
    let vector :Vec<i64> = line.split(" ")
        .map(|s| s.parse::<i64>().unwrap()).collect();
    vector
}

fn parse_input(lines: &mut Lines<StdinLock>) -> Vec<(u64, Vec<i64>, Vec<i64>)> {
    let number_test_cases = lines.next().unwrap().unwrap()
        .parse::<u64>().unwrap();
    let mut test_cases = Vec::new();

    for _ in 0..number_test_cases {
        let size = lines.next().unwrap().unwrap()
            .parse::<u64>().unwrap();
        let first_vector = parse_vector(&lines.next().unwrap().unwrap());
        let second_vector = parse_vector(&lines.next().unwrap().unwrap());
        test_cases.push((size, first_vector, second_vector));
    }

    test_cases
}

fn minimum_scalar_product(test_case: &(u64, Vec<i64>, Vec<i64>)) -> i64 {
    let &(_, ref first_vector, ref second_vector) = test_case;
    let mut sorted_first = first_vector.to_vec();
    sorted_first.sort();
    let mut sorted_second = second_vector.to_vec();
    sorted_second.sort_by(|a, b| b.cmp(a));
    let scalar_product = sorted_first.iter().zip(sorted_second.iter())
        .map(|(x, y)| x * y).collect::<Vec<i64>>().iter()
        .fold(0, |acc, &item| acc + item);
    scalar_product
}

fn process_test_cases(test_cases: &Vec<(u64, Vec<i64>, Vec<i64>)>) {
    let mut counter = 1;
    for test_case in test_cases {
        let msp = minimum_scalar_product(test_case);
        println!("Case #{}: {}", counter, msp);
        counter += 1;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let parsed_input = parse_input(&mut lines);
    process_test_cases(&parsed_input);
}
