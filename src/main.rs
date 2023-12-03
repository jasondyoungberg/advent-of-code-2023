mod day1;

fn main() {
    println!("Day 1: {}", day1::solve(include_str!("day1.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        let data = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let result = 142;
        assert_eq!(day1::solve(data), result);
    }
}
