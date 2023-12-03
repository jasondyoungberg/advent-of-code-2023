pub fn solve(data: &str) -> i32 {
    data.lines().map(|line| {
        let numbers: Vec<_> = line
            .chars()
            .filter_map(|x| x.to_digit(10))
            .collect();
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
    
        i32::try_from(first*10 + last).unwrap()
    }).sum()
}
