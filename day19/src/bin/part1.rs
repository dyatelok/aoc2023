use std::collections::HashMap;

use itertools::*;

fn split3(input: &str, id: usize) -> (&str, char, &str) {
    (&input[..id], input.as_bytes()[id] as char, &input[id + 1..])
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

impl Part {
    fn sum(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
    fn get(&self, variable: Variable) -> i32 {
        match variable {
            Variable::X => self.x,
            Variable::M => self.m,
            Variable::A => self.a,
            Variable::S => self.s,
        }
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

impl Workflow {
    fn call(&self, arg: Part) -> Return {
        for rule in &self.rules {
            match rule.condition {
                None => {
                    return rule.ret.clone();
                }
                Some((var, comp, val)) => {
                    let var = arg.get(var);
                    if comp.res(var, val) {
                        return rule.ret.clone();
                    }
                }
            }
        }

        panic!("Function did not terminate!")
    }
}

#[derive(Clone, Debug)]
struct Rule {
    condition: Option<(Variable, Comparator, i32)>,
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
                let val = val.parse::<i32>().unwrap();
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

impl Comparator {
    fn res(&self, a: i32, b: i32) -> bool {
        match self {
            Self::Bigger => a > b,
            Self::Smaller => a < b,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Return {
    Accept,
    Reject,
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
    fn accepts(&self, part: &Part) -> bool {
        self.call(String::from("in"), part) == Return::Accept
    }
    fn call(&self, name: String, arg: &Part) -> Return {
        let workflow = self.workflows.get(&name).unwrap();

        match workflow.call(*arg) {
            Return::Call(wname) => self.call(wname, arg),
            ret => ret,
        }
    }
}

fn solution(input: &str) -> i32 {
    let (workflows, inputs) = input.split("\n\n").next_tuple().unwrap();

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

    inputs
        .lines()
        .map(Part::from)
        .filter(|part| handler.accepts(part))
        .map(|part| part.sum())
        .sum()
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

        assert_eq!(res, 19114);
    }
}
