fn split3(input: &str, id: usize) -> (&str, char, &str) {
    (&input[..id], input.as_bytes()[id] as char, &input[id + 1..])
}

struct Action {
    id: usize,
    op: Oper,
}

#[derive(Debug, Clone, Copy)]
enum Oper {
    Remove,
    Add(usize),
}

impl Action {
    fn from_ret(input: &str) -> (Self, String) {
        let pos = input.find('-').unwrap_or(0) + input.find('=').unwrap_or(0);
        let (label, code, num) = split3(input, pos);

        let id = hash(label);

        (
            match code {
                '-' => Action {
                    id,
                    op: Oper::Remove,
                },
                '=' => Action {
                    id,
                    op: Oper::Add(num.parse::<usize>().unwrap()),
                },
                _ => panic!(),
            },
            label.to_owned(),
        )
    }
}

#[derive(Clone, Debug)]
struct LensBox(Vec<Lens>);

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    power: usize,
}

impl LensBox {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn remove(&mut self, label: String) {
        self.0.retain(|lens| lens.label != label);
    }
    fn add(&mut self, label: String, power: usize) {
        if let Some((id, _)) = self.0.iter().enumerate().find(
            |(
                _,
                Lens {
                    label: lens_label, ..
                },
            )| *lens_label == label,
        ) {
            self.0[id].power = power;
        } else {
            self.0.push(Lens { label, power });
        }
    }
    fn calc(self) -> usize {
        self.0
            .into_iter()
            .enumerate()
            .fold(0, |acc, curr| acc + (curr.0 + 1) * curr.1.power)
    }
}

fn hash(input: &str) -> usize {
    input
        .trim()
        .chars()
        .map(|ch| ch as u8)
        .fold(0, |acc, v| ((acc + v as usize) * 17) % 256)
}

fn solution(input: &str) -> usize {
    let mut boxes = vec![LensBox::new(); 256];

    input
        .trim()
        .split(',')
        .map(Action::from_ret)
        .for_each(|(Action { id, op }, label)| {
            // println!("id: {}, op: {:?}", id, op);
            match op {
                Oper::Remove => boxes[id].remove(label),
                Oper::Add(power) => boxes[id].add(label, power),
            }
            // println!("{:#?}", &boxes[..4]);
        });

    boxes
        .into_iter()
        .enumerate()
        .map(|(i, lens_box)| lens_box.calc() * (i + 1))
        .sum()
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = solution(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use crate::{hash, solution};

    #[test]
    fn test() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let res = solution(input);

        assert_eq!(res, 145);
    }
    #[test]
    fn test_hash() {
        let input = "rn";
        let res = hash(input);
        assert_eq!(res, 0);

        let input = "cm";
        let res = hash(input);
        assert_eq!(res, 0);

        let input = "qp";
        let res = hash(input);
        assert_eq!(res, 1);

        let input = "pc";
        let res = hash(input);
        assert_eq!(res, 3);

        let input = "ab";
        let res = hash(input);
        assert_eq!(res, 3);
    }
}
