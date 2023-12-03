fn split_on(line: &str, mid: usize) -> (&str, &str) {
    (&line[..mid], &line[mid + 1..])
}

fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, takes) = split_on(line, line.find(':').unwrap());
            let (r, g, b) = takes
                .split(';')
                .map(|take| {
                    let (mut r, mut g, mut b) = (0, 0, 0);
                    take.split(',').for_each(|item| {
                        let mut splitted = item.split_whitespace();
                        let (num, color) = (
                            splitted.next().unwrap().parse::<u32>().unwrap(),
                            splitted.next().unwrap(),
                        );
                        match (color, num) {
                            ("red", n) => r += n,
                            ("green", n) => g += n,
                            ("blue", n) => b += n,
                            _ => {}
                        };
                    });
                    (r, g, b)
                })
                .fold((0, 0, 0), |(rp, gp, bp), (r, g, b)| {
                    (rp.max(r), gp.max(g), bp.max(b))
                });
            r * g * b
        })
        .sum::<u32>()
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
    fn exploration() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let res = solution(input);
        assert_eq!(res, 2286);
    }
}
