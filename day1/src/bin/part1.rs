fn main() {
    let input = include_str!("../../input1.txt");
    let res = input
        .lines()
        .map(|line| {
            let vec: Vec<u32> = line.chars().filter_map(|elem| elem.to_digit(10)).collect();
            vec.first().unwrap() * 10 + vec.last().unwrap()
        })
        .sum::<u32>();
    println!("{res}");
}
