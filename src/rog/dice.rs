use std::vec;

use super::rogcons::*;
use rand::random;
use std::result::Result;

pub enum Explode {
    NoExplode,
    Explode(usize),
    Default,
}

pub struct RollConfig {
    pub keep_drop: Option<(KeepDrop, usize)>,
    pub explode: Explode,
    pub aro: bool,
    pub sort: bool,
}
impl RollConfig {
    pub fn new() -> Self {
        Self {
            keep_drop: None,
            explode: Explode::NoExplode,
            aro: false,
            sort: false,
        }
    }
}
impl ToString for RollConfig {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}",
            match self.explode {
                Explode::NoExplode => String::new(),
                Explode::Default => String::from("!"),
                Explode::Explode(x) => format!("!{}", x),
            },
            if let Some(kd) = &self.keep_drop {
                format!(
                    "{}{}",
                    match kd.0 {
                        KeepDrop::Crit => "c",
                        KeepDrop::DropHigh => "dh",
                        KeepDrop::DropLow => "d",
                        KeepDrop::KeepHigh => "k",
                        KeepDrop::KeepLow => "kl",
                    },
                    kd.1
                )
            } else {
                String::new()
            },
            if self.aro { "aro" } else { "" },
            if self.sort { "s" } else { "" }
        )
    }
}

#[derive(PartialEq, Eq)]
pub enum KeepDrop {
    KeepHigh,
    KeepLow,
    DropHigh,
    DropLow,
    Crit,
}

pub struct Dice {
    pub count: usize,
    pub sides: usize,
    pub config: RollConfig,
}
impl Dice {
    pub fn new() -> Self {
        Self {
            count: 1,
            sides: 6,
            config: RollConfig::new(),
        }
    }
    fn die_formatter(value: f64, crit: f64, kept: bool) -> String {
        let mut result = format!("{:.0}", value);
        if value == 1. || value >= crit {
            result = format!("**{}**", result);
        }
        if !kept {
            result = format!("~~{}~~", result);
        }
        result
    }
    fn single_roll(sides: usize, explode_size: Option<usize>) -> Vec<f64> {
        let mut values = vec![];
        let mut value: f64;
        loop {
            value = (random::<usize>() % sides + 1) as f64;
            values.push(value);
            if let Some(explode) = explode_size {
                if value < explode as f64 {
                    break;
                }
            } else {
                break;
            }
        }
        values
    }
    pub fn roll(self) -> Result<RogCons, RogErr> {
        if self.count > 100 {
            return Err(RogErr::CountMax);
        }
        if self.sides < 2 {
            return Err(RogErr::CountMin);
        }
        let sort = self.config.sort || self.config.keep_drop.is_some();
        let explode_size = match self.config.explode {
            Explode::Default => Some(self.sides),
            Explode::Explode(ex) => Some(ex),
            Explode::NoExplode => None,
        };

        if let Some(size) = explode_size {
            if size < 2 {
                return Err(RogErr::ExplodeMin);
            }
            if size < self.sides / 100 {
                return Err(RogErr::ExplodeChanceHigh);
            }
        }

        let mut values: Vec<f64> = vec![];
        for _ in 0..self.count {
            values.append(&mut Self::single_roll(self.sides, explode_size));
        }

        if self.config.aro && values.len() > 1 {
            let first = values[0];
            let mut all_same = values.iter().all(|v| v == &first);
            while all_same {
                let mut new_values = Self::single_roll(self.sides, explode_size);
                all_same = new_values.iter().all(|v| v == &first);
                values.append(&mut new_values);
            }
        }

        let values: Vec<(f64, bool)> = values.into_iter().map(|x| (x, true)).collect();
        let mut sorted_values: Vec<(f64, bool)> = values.clone().into_iter().collect();

        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted_values.reverse();

        let mut keep_range = 0..sorted_values.len();
        let mut crit_value = self.sides as f64;
        if let Some((keep_drop, keep_value)) = &self.config.keep_drop {
            let keep_value = usize::clamp(*keep_value, 0, self.count as usize);
            match keep_drop {
                KeepDrop::KeepHigh => keep_range.end = keep_value,
                KeepDrop::KeepLow => keep_range.start = sorted_values.len() - keep_value,
                KeepDrop::DropHigh => keep_range.start = keep_value,
                KeepDrop::DropLow => keep_range.end = sorted_values.len() - keep_value,
                KeepDrop::Crit => crit_value = keep_value as f64,
            }
            sorted_values.iter_mut().enumerate().for_each(|(i, value)| {
                if !keep_range.contains(&i) {
                    *value = (value.0, false)
                }
            });
        }

        let final_values = if sort { sorted_values } else { values };
        let accumulated_value = final_values
            .iter()
            .filter(|(_, kept)| *kept)
            .map(|(x, _)| x)
            .sum();

        let mut text = final_values
            .iter()
            .map(|(v, keep)| Self::die_formatter(*v, crit_value, *keep))
            .collect::<Vec<_>>()
            .join(", ");
        text = format!(
            "[{}] {}d{}{}",
            text,
            self.count,
            self.sides,
            self.config.to_string()
        );

        let final_values: Vec<f64> = final_values.into_iter().map(|(v, _)| v).collect();

        Ok(RogCons {
            value: accumulated_value,
            text,
            values: final_values,
            boolean: false,
        })
    }
}

pub struct FateDice {
    pub count: usize,
}

impl FateDice {
    pub fn new() -> Self {
        Self { count: 1 }
    }
    fn format_dice(x: &f64) -> String {
        if x == &1. {
            "**+**".to_string()
        } else if x == &0. {
            "0".to_string()
        } else if x == &-1. {
            "**-**".to_string()
        } else {
            panic!("Bad item in fate dice: {}", x);
        }
    }
    pub fn roll(self) -> Result<RogCons, RogErr> {
        if self.count > 100 {
            return Err(RogErr::CountMax);
        }
        let values: Vec<f64> = (0..self.count)
            .map(|_| (random::<u32>() % 3) as f64 - 1.)
            .collect();

        let mut text = values
            .iter()
            .map(Self::format_dice)
            .collect::<Vec<String>>()
            .join(", ");
        text = format!("[{}] {}df", text, self.count);

        Ok(RogCons {
            value: values.iter().sum(),
            text,
            values,
            boolean: false,
        })
    }
}

#[derive(Debug)]
pub enum RogErr {
    CountMax,
    CountMin,
    ExplodeMin,
    ExplodeChanceHigh,
    InvalidAttribute,
}
