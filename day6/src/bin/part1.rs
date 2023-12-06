fn split_on(line: &str, mid: usize) -> (&str, &str) {
    (&line[..mid], &line[mid + 1..])
}

fn get_data(input: &str) -> Vec<i32> {
    let (_, ranges) = split_on(input, input.find(':').unwrap());

    ranges
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect()
}

fn solution(input: &str) -> i32 {
    let mut iter = input.lines();
    let times = get_data(iter.next().unwrap());
    let distanse = get_data(iter.next().unwrap());

    let time_dis = times.into_iter().zip(distanse);

    time_dis
        .map(|(time, distance)| {
            let d = (time.pow(2) - 4 * distance) as f32;
            if d <= 0.0 {
                0
            } else {
                let lower = (time as f32 - d.sqrt()) / 2.0 + 1.0;
                let lower = lower.floor() as i32;

                let upper = (time as f32 + d.sqrt()) / 2.0 - 1.0;
                let upper = upper.ceil() as i32;

                upper - lower + 1
            }
        })
        .product()
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
        assert_eq!(res, 288);
    }
}
