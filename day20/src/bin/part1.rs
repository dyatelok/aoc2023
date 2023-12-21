use itertools::*;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
struct Module {
    outputs: Vec<String>,
    ty: Type,
}

#[derive(Clone, Debug)]
enum Type {
    Broadcaster,
    FlipFlop { on: bool },
    Conjunction { inputs: HashMap<String, Pulse> },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pulse {
    High,
    Low,
}

struct Propagator {
    modules: HashMap<String, Module>,
    stack: VecDeque<(Option<String>, String, Pulse)>,
    counter_low: u64,
    counter_high: u64,
}

impl Propagator {
    fn from(modules: Vec<(String, Module)>) -> Self {
        let mut map: HashMap<_, _> = modules.clone().into_iter().collect();
        for (name, module) in &modules {
            for output in &module.outputs {
                if let Some(Module {
                    outputs: _,
                    ty: Type::Conjunction { inputs },
                }) = map.get_mut(output)
                {
                    inputs.insert(name.clone(), Pulse::Low);
                }
            }
        }
        Self {
            modules: map,
            stack: VecDeque::new(),
            counter_low: 0,
            counter_high: 0,
        }
    }
    fn press(&mut self) {
        self.stack
            .push_back((None, String::from("broadcaster"), Pulse::Low));

        while let Some((from, to, pulse)) = self.stack.pop_front() {
            self.send(from, to, pulse);
        }
    }
    fn send(&mut self, from: Option<String>, to: String, pulse: Pulse) {
        match pulse {
            Pulse::High => self.counter_high += 1,
            Pulse::Low => self.counter_low += 1,
        }

        let Some(module) = self.modules.get_mut(&to) else {
            // println!("{}", to);
            return;
        };

        match (pulse, &mut module.ty) {
            (pulse, Type::Broadcaster) => {
                for output in &module.outputs {
                    self.stack
                        .push_back((Some(to.clone()), output.clone(), pulse));
                }
            }
            (Pulse::High, Type::FlipFlop { .. }) => {}
            (Pulse::Low, Type::FlipFlop { on }) => {
                let pulse = if *on { Pulse::Low } else { Pulse::High };
                for output in &module.outputs {
                    self.stack
                        .push_back((Some(to.clone()), output.clone(), pulse));
                }
                *on = !*on;
            }
            (pulse, Type::Conjunction { inputs }) => {
                inputs.insert(from.unwrap(), pulse);

                let pulse = if inputs.iter().all(|(_, pulse)| *pulse == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                for output in &module.outputs {
                    self.stack
                        .push_back((Some(to.clone()), output.clone(), pulse));
                }
            }
        }
    }
    fn compute(&mut self) -> u64 {
        for _ in 0..1000 {
            self.press();
        }
        // println!("low: {}  high: {}", self.counter_low, self.counter_high);
        self.counter_low * self.counter_high
    }
}

fn solution(input: &str) -> u64 {
    let modules = input
        .lines()
        .map(|line| {
            let ty = match line.chars().next().unwrap() {
                '%' => Type::FlipFlop { on: false },
                '&' => Type::Conjunction {
                    inputs: HashMap::new(),
                },
                _ => Type::Broadcaster,
            };

            let mut iter = line.split("->");

            let name = iter
                .next()
                .unwrap()
                .trim()
                .to_owned()
                .replace(['%', '&'], "");

            let outputs = iter
                .next()
                .unwrap()
                .split(',')
                .map(|entry| entry.trim().to_owned())
                .collect_vec();

            (name, Module { outputs, ty })
        })
        .collect_vec();

    Propagator::from(modules).compute()
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
    fn test1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let res = solution(input);

        assert_eq!(res, 32000000);
    }
    #[test]
    fn test2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let res = solution(input);

        assert_eq!(res, 11687500);
    }
}
