fn hash(input: &str) -> u32 {
    input
        .trim()
        .chars()
        .map(|ch| ch as u8)
        .fold(0, |acc, v| ((acc + v as u32) * 17) % 256)
}

fn solution(input: &str) -> u32 {
    input.split(',').map(hash).sum()
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = solution(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn test() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let res = solution(input);

        assert_eq!(res, 1320);
    }
}
