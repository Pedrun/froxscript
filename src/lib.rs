mod rog;
use std::collections::HashMap;

//use std::{fs, time::SystemTime};
use napi_derive::napi;
use rog::*;

#[derive(Parser)]
#[grammar = "rog.pest"]
struct RogParser;

use pest::{iterators::Pairs, pratt_parser::*, Parser};
use pest_derive::Parser;

fn parse_expression(
    pairs: Pairs<Rule>,
    pratt: &PrattParser<Rule>,
    attr_map: &HashMap<String, f64>,
) -> Result<RogCons, RogErr> {
    let result = pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Ok(RogCons::from_number(
                parse_number(primary.as_str(), attr_map)?,
                String::new(),
            )),
            Rule::attribute => {
                let res = parse_number(primary.as_str(), attr_map)?;
                Ok(RogCons::from_number(
                    res,
                    format!("[{}] {}", res, primary.as_str()),
                ))
            }
            Rule::expression => Ok(parse_expression(primary.into_inner(), pratt, attr_map)?),
            Rule::group => {
                let mut exp = parse_expression(primary.into_inner(), pratt, attr_map)?;
                exp.text = format!("({})", exp.text);
                Ok(exp)
            }
            Rule::fate_dice => roll_fate(primary.into_inner(), attr_map),
            Rule::dice => roll_dice(primary.into_inner(), attr_map),
            _ => return Err(RogErr::UnknownError),
        })
        .map_infix(|lhs, op, rhs| {
            let lhs = lhs?;
            let rhs = rhs?;
            Ok(match op.as_rule() {
                Rule::and => lhs & rhs,
                Rule::or => lhs | rhs,
                Rule::less_eq => lhs.less_eq(rhs),
                Rule::less => lhs.less(rhs),
                Rule::greater_eq => lhs.greater_eq(rhs),
                Rule::greater => lhs.greater(rhs),
                Rule::eq => lhs.eq(rhs),
                Rule::hyper_add => lhs.hyper_add(rhs),
                Rule::hyper_sub => lhs.hyper_sub(rhs),
                Rule::counter_less => lhs << rhs,
                Rule::counter_greater => lhs >> rhs,
                Rule::add => lhs + rhs,
                Rule::sub => lhs - rhs,
                Rule::mul => lhs * rhs,
                Rule::div => lhs / rhs,
                _ => return Err(RogErr::UnknownError),
            })
        })
        .map_prefix(|op, rhs| {
            let rhs = rhs?;
            Ok(match op.as_rule() {
                Rule::neg => -rhs,
                Rule::not => !rhs,
                _ => return Err(RogErr::UnknownError),
            })
        })
        .map_postfix(|lhs, op| {
            let lhs = lhs?;
            Ok(match op.as_rule() {
                Rule::percent => lhs.percent(),
                _ => return Err(RogErr::UnknownError),
            })
        })
        .parse(pairs);
    result
}

fn parse_number(text: &str, attr_map: &HashMap<String, f64>) -> Result<f64, RogErr> {
    let a_start = text.find(|c| match c {
        'A'..='Z' | '$' => true,
        _ => false,
    });
    Ok(if let Some(split) = a_start {
        if split == 0 {
            *attr_map.get(text).ok_or(RogErr::InvalidAttribute)?
        } else {
            let (num, attr) = text.split_at(split);
            let num = parse_float(num);
            let attr = attr_map.get(attr).ok_or(RogErr::InvalidAttribute)?;
            num * attr
        }
    } else {
        parse_float(text)
    })
}

fn parse_float(float: &str) -> f64 {
    float
        .parse()
        .expect(&format!("Expected float, got {}", float))
}

fn roll_dice(pairs: Pairs<Rule>, attr_map: &HashMap<String, f64>) -> Result<RogCons, RogErr> {
    let mut dice = Dice::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::dice_n => {
                if !pair.as_str().is_empty() {
                    dice.count = parse_number(pair.as_str(), attr_map)? as usize
                }
            }
            Rule::dice_side => dice.sides = parse_number(pair.as_str(), attr_map)? as usize,
            Rule::roll_config => dice.config = roll_config(pair.into_inner(), attr_map)?,
            _ => unreachable!(),
        }
    }
    dice.roll()
}

fn roll_config(pairs: Pairs<Rule>, attr_map: &HashMap<String, f64>) -> Result<RollConfig, RogErr> {
    let mut config = RollConfig::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::aro => config.aro = true,
            Rule::sort => config.sort = true,
            Rule::keep_drop => {
                config.keep_drop = Some(keep_drop_config(pair.into_inner(), attr_map)?)
            }
            Rule::explode => {
                config.explode = pair.into_inner().next().map_or(Ok(Explode::Default), |x| {
                    Ok(Explode::Explode(
                        parse_number(x.as_str(), attr_map)? as usize
                    ))
                })?
            }
            _ => unreachable!(),
        }
    }
    Ok(config)
}

fn keep_drop_config(
    pairs: Pairs<Rule>,
    attr_map: &HashMap<String, f64>,
) -> Result<(KeepDrop, usize), RogErr> {
    let mut iter = pairs.into_iter();
    let keep_drop = match iter.next().ok_or(RogErr::UnknownError)?.as_rule() {
        Rule::keep_high => KeepDrop::KeepHigh,
        Rule::keep_low => KeepDrop::KeepLow,
        Rule::drop_high => KeepDrop::DropHigh,
        Rule::drop_low => KeepDrop::DropLow,
        Rule::crit => KeepDrop::Crit,
        _ => unreachable!(),
    };
    let value = parse_number(iter.next().ok_or(RogErr::UnknownError)?.as_str(), attr_map)? as usize;
    Ok((keep_drop, value))
}

fn roll_fate(pairs: Pairs<Rule>, attr_map: &HashMap<String, f64>) -> Result<RogCons, RogErr> {
    let mut dice = FateDice::new();
    let pair = pairs.into_iter().next().ok_or(RogErr::UnknownError)?;
    match pair.as_rule() {
        Rule::dice_n => {
            if !pair.as_str().is_empty() {
                dice.count = parse_number(pair.as_str(), attr_map)? as usize
            }
        }
        _ => unreachable!(),
    }
    dice.roll()
}

fn get_parser() -> PrattParser<Rule> {
    PrattParser::new()
        .op(Op::infix(Rule::and, Assoc::Left))
        .op(Op::infix(Rule::or, Assoc::Left))
        .op(Op::infix(Rule::counter_less, Assoc::Left)
            | Op::infix(Rule::counter_greater, Assoc::Left))
        .op(Op::infix(Rule::less, Assoc::Left)
            | Op::infix(Rule::less_eq, Assoc::Left)
            | Op::infix(Rule::greater, Assoc::Left)
            | Op::infix(Rule::greater_eq, Assoc::Left)
            | Op::infix(Rule::eq, Assoc::Left))
        .op(Op::infix(Rule::add, Assoc::Left)
            | Op::infix(Rule::sub, Assoc::Left)
            | Op::infix(Rule::hyper_add, Assoc::Left)
            | Op::infix(Rule::hyper_sub, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left)
            | Op::infix(Rule::div, Assoc::Left)
            | Op::infix(Rule::and, Assoc::Left)
            | Op::infix(Rule::or, Assoc::Left))
        .op(Op::postfix(Rule::percent))
        .op(Op::prefix(Rule::neg) | Op::prefix(Rule::not))
}

fn parse_assignment(
    pairs: Pairs<Rule>,
    pratt: &PrattParser<Rule>,
    attr_map: &mut HashMap<String, f64>,
) -> Result<RogCons, RogErr> {
    let mut it = pairs.rev();
    let expression_pairs = it.next().ok_or(RogErr::UnknownError)?.into_inner();
    let mut result = parse_expression(expression_pairs, pratt, attr_map)?;

    let assignment_pairs = it.next();
    if let Some(assign) = assignment_pairs {
        let key = assign.as_str().trim().to_string();
        if !key.starts_with('$') && !attr_map.contains_key(&key) {
            return Err(RogErr::InvalidAttribute);
        }
        attr_map.insert(key.clone(), result.value);
        if attr_map.len() > 100 {
            return Err(RogErr::AttributeMax);
        }
        result.text = format!("{} := {}", key, result.text);
    }
    result.text = format!("` {} ` ⟵ {}", result.value, result.text);
    Ok(result)
}

fn parse_repeat(
    pairs: Pairs<Rule>,
    pratt: &PrattParser<Rule>,
    attr_map: &mut HashMap<String, f64>,
    buf: &mut Vec<RogCons>,
) -> Result<(), RogErr> {
    let it = pairs
        .into_iter()
        .next()
        .ok_or(RogErr::UnknownError)?
        .into_inner();

    let mut repeat_count: usize = 1;
    let mut comment = None;
    let mut assignment = None;
    for pair in it {
        match pair.as_rule() {
            Rule::repeat_literal => {
                let input = pair.as_str().trim();
                repeat_count = input
                    .strip_suffix("#")
                    .unwrap_or(input)
                    .parse()
                    .map_err(|_| RogErr::UnknownError)?;
            }
            Rule::comment => comment = pair.as_str().strip_prefix(";"),
            Rule::assignment => assignment = Some(pair.into_inner()),
            Rule::EOI => {
                continue;
            }
            r => println!("{:?}", r),
        }
    }

    let assignment = assignment.ok_or(RogErr::UnknownError)?;
    for _ in 0..repeat_count {
        if buf.len() >= 100 {
            return Err(RogErr::LineMax);
        }
        let mut current_cons = parse_assignment(assignment.clone(), pratt, attr_map)?;
        if let Some(c) = comment {
            current_cons.text = format!("**{}** {}", c.trim(), current_cons.text);
        }
        buf.push(current_cons);
    }

    Ok(())
}

#[napi(object)]
#[derive(Debug)]
pub struct Output {
    pub cons: Vec<RogCons>,
    pub attr_map: HashMap<String, f64>,
}

#[napi]
pub fn parse(input: String, mut attr_map: HashMap<String, f64>) -> Option<Output> {
    let pratt = get_parser();

    let inputs = input.split("\n").map(str::trim).filter(|i| !i.is_empty());

    let mut cons = vec![];
    for input in inputs {
        let pairs = RogParser::parse(Rule::repeat, &input).ok()?;
        parse_repeat(pairs, &pratt, &mut attr_map, &mut cons).ok()?;
    }

    Some(Output { cons, attr_map })
}

#[test]
fn test() {
    let map = HashMap::from([
        (String::from("A"), 10.),
        (String::from("B"), 15.),
        (String::from("C"), 392.),
        (String::from("LONGO"), 8.),
    ]);
    let out = parse("101#d20".to_string(), map);
    if let Some(out) = out {
        let c = out
            .cons
            .into_iter()
            .map(|r| r.text)
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", c);
    } else {
        println!("{:?}", out);
    }
}
