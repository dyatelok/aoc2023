use std::collections::HashMap;

use itertools::*;

fn split3(input: &str, id: usize) -> (&str, char, &str) {
    (&input[..id], input.as_bytes()[id] as char, &input[id + 1..])
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn new() -> Self {
        Self {
            x: Range::from(1, 4000),
            m: Range::from(1, 4000),
            a: Range::from(1, 4000),
            s: Range::from(1, 4000),
        }
    }
    fn from(x: Range, m: Range, a: Range, s: Range) -> Self {
        Self { x, m, a, s }
    }
    fn count(&self) -> i64 {
        self.x.length() * self.m.length() * self.a.length() * self.s.length()
    }
    fn split(self, (var, cond, val): (Variable, Comparator, i64)) -> (Option<Self>, Option<Self>) {
        let to_split = match var {
            Variable::X => self.x,
            Variable::M => self.m,
            Variable::A => self.a,
            Variable::S => self.s,
        };

        let (processes, denies) = match cond {
            Comparator::Bigger => {
                if to_split.start > val {
                    (Some(to_split), None)
                } else if to_split.end <= val {
                    (None, Some(to_split))
                } else {
                    (
                        Some(Range::from(val + 1, to_split.end)),
                        Some(Range::from(to_split.start, val)),
                    )
                }
            }
            Comparator::Smaller => {
                if to_split.end < val {
                    (Some(to_split), None)
                } else if to_split.start >= val {
                    (None, Some(to_split))
                } else {
                    (
                        Some(Range::from(to_split.start, val - 1)),
                        Some(Range::from(val, to_split.end)),
                    )
                }
            }
        };

        let (x, m, a, s) = (self.x, self.m, self.a, self.s);

        match var {
            Variable::X => (
                processes.map(|range| Self::from(range, m, a, s)),
                denies.map(|range| Self::from(range, m, a, s)),
            ),
            Variable::M => (
                processes.map(|range| Self::from(x, range, a, s)),
                denies.map(|range| Self::from(x, range, a, s)),
            ),
            Variable::A => (
                processes.map(|range| Self::from(x, m, range, s)),
                denies.map(|range| Self::from(x, m, range, s)),
            ),
            Variable::S => (
                processes.map(|range| Self::from(x, m, a, range)),
                denies.map(|range| Self::from(x, m, a, range)),
            ),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn from(start: i64, end: i64) -> Self {
        Self { start, end }
    }
    fn length(&self) -> i64 {
        self.end - self.start + 1
    }
    // fn split(&self, ) -> Vec<Range> {
    //     todo!()
    // }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let mut sep = value.split(',').map(clear_to_num);
        let (x, m, a, s) = sep.next_tuple().unwrap();
        Self { x, m, a, s }
    }
}

fn clear_to_num(inp: &str) -> i32 {
    inp.chars()
        .filter(|ch| ch.is_ascii_digit())
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

#[derive(Clone, Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        Self {
            rules: value.split(',').map(Rule::from).collect(),
        }
    }
}

#[derive(Clone, Debug)]
struct Rule {
    condition: Option<(Variable, Comparator, i64)>,
    ret: Return,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        match value.find(':') {
            Some(id) => {
                let (cond, _, ret) = split3(value, id);
                let id = cond.find('>').unwrap_or(0) + cond.find('<').unwrap_or(0);
                let (var, comp, val) = split3(cond, id);
                let var = Variable::from(var);
                let comp = Comparator::from(comp);
                let val = val.parse::<i64>().unwrap();
                Self {
                    condition: Some((var, comp, val)),
                    ret: Return::from(ret),
                }
            }
            None => Self {
                condition: None,
                ret: Return::from(value),
            },
        }
    }
}

impl Rule {
    fn split(&self, arg: PartRange) -> Vec<(PartRange, Return)> {
        match self.condition {
            None => {
                vec![(arg, self.ret.clone())]
            }
            Some(conf) => {
                let (processes, denies) = arg.split(conf);
                vec![(processes, self.ret.clone()), (denies, Return::Next)]
                    .into_iter()
                    .filter_map(|(range, ret)| range.map(|range| (range, ret)))
                    .collect()
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Variable {
    X,
    M,
    A,
    S,
}

impl From<&str> for Variable {
    fn from(value: &str) -> Self {
        match value {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Comparator {
    Bigger,
    Smaller,
}

impl From<char> for Comparator {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::Bigger,
            '<' => Self::Smaller,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Return {
    Accept,
    Reject,
    Next,
    Call(String),
}

impl From<&str> for Return {
    fn from(value: &str) -> Self {
        match value {
            "A" => Return::Accept,
            "R" => Return::Reject,
            _ => Return::Call(value.to_owned()),
        }
    }
}

struct Handler {
    workflows: HashMap<String, Workflow>,
}

impl Handler {
    fn from(workflows: HashMap<String, Workflow>) -> Self {
        Self { workflows }
    }
    fn call(&self, name: &String, index: usize, arg: PartRange) -> i64 {
        let workflow = self.workflows.get(name).unwrap();
        let rule = workflow.rules.get(index).unwrap();

        rule.split(arg)
            .into_iter()
            .map(|(range, ret)| match ret {
                Return::Accept => range.count(),
                Return::Reject => 0,
                Return::Next => self.call(name, index + 1, range),
                Return::Call(wname) => self.call(&wname, 0, range),
            })
            .sum()
    }
    fn count(&self) -> i64 {
        let full = PartRange::new();
        self.call(&String::from("in"), 0, full)
    }
}

fn solution(input: &str) -> i64 {
    let (workflows, _) = input.split("\n\n").next_tuple().unwrap();

    let workflows: HashMap<String, Workflow> = workflows
        .lines()
        .map(|line| {
            let name_end = line.find('{').unwrap();
            (
                String::from(&line[..name_end]),
                Workflow::from(&line[name_end + 1..line.len() - 1]),
            )
        })
        .collect();

    let handler = Handler::from(workflows);

    handler.count()
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
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let res = solution(input);

        assert_eq!(res, 167409079868000);
    }
}
