fn split_on(line: &str, mid: usize) -> (&str, &str) {
    (&line[..mid], &line[mid + 1..])
}

fn get_data(input: &str) -> u128 {
    let (_, nums) = split_on(input, input.find(':').unwrap());

    let nums: String = nums.split_whitespace().fold(String::new(), |mut s1, s2| {
        s1.push_str(s2);
        s1
    });

    nums.parse::<u128>().unwrap()
}

fn solution(input: &str) -> u128 {
    let mut iter = input.lines();
    let time = get_data(iter.next().unwrap());
    let distanse = get_data(iter.next().unwrap());

    let d = (time.pow(2) - 4 * distanse) as f64;
    if d <= 0.0 {
        0
    } else {
        let lower = (time as f64 - d.sqrt()) / 2.0 + 1.0;
        let lower = lower.floor() as u128;

        let upper = (time as f64 + d.sqrt()) / 2.0 - 1.0;
        let upper = upper.ceil() as u128;

        upper - lower + 1
    }
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = solution(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let res = solution(input);
        assert_eq!(res, 71503);
    }
}
