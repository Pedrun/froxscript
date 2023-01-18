use std::vec;

use napi_derive::napi;

#[derive(Debug)]
#[napi(object)]
pub struct RogCons {
    pub value: f64,
    pub values: Vec<f64>,
    pub text: String,
    pub boolean: bool,
    pub dice: u32,
}

impl RogCons {
    pub fn from_number(value: f64, text: String) -> Self {
        Self {
            value,
            values: vec![value],
            text: if text.is_empty() {
                value.to_string()
            } else {
                text
            },
            boolean: false,
            dice: 0,
        }
    }
    pub fn hyper_add(mut self, rhs: Self) -> Self {
        self.value += rhs.value * self.values.len() as f64;
        self.values.iter_mut().for_each(|v| *v += rhs.value);
        let joined = self
            .values
            .iter()
            .map(|x| format!("{:.0}", x))
            .collect::<Vec<String>>()
            .join(", ");
        self.text = format!("[{}] ⟵ {} ++ {}", joined, self.text, rhs.text);
        self.boolean = false;
        self.dice += rhs.dice;
        self
    }
    pub fn hyper_sub(mut self, rhs: Self) -> Self {
        self.value -= rhs.value * self.values.len() as f64;
        self.values.iter_mut().for_each(|v| *v -= rhs.value);
        let joined = self
            .values
            .iter()
            .map(|x| format!("{:.0}", x))
            .collect::<Vec<String>>()
            .join(", ");
        self.text = format!("[{}] ⟵ {} -- {}", joined, self.text, rhs.text);
        self.boolean = false;
        self.dice += rhs.dice;
        self
    }
    pub fn less(self, rhs: Self) -> Self {
        let value = if self.value < rhs.value { 1. } else { 0. };
        let mut cons = Self::from_number(value, format!("{} < {}", self.text, rhs.text));
        cons.boolean = true;
        cons.dice = self.dice + rhs.dice;
        cons
    }
    pub fn less_eq(self, rhs: Self) -> Self {
        let value = if self.value <= rhs.value { 1. } else { 0. };
        let mut cons = Self::from_number(value, format!("{} <= {}", self.text, rhs.text));
        cons.boolean = true;
        cons.dice = self.dice + rhs.dice;
        cons
    }
    pub fn greater(self, rhs: Self) -> Self {
        let value = if self.value > rhs.value { 1. } else { 0. };
        let mut cons = Self::from_number(value, format!("{} > {}", self.text, rhs.text));
        cons.boolean = true;
        cons.dice = self.dice + rhs.dice;
        cons
    }
    pub fn greater_eq(self, rhs: Self) -> Self {
        let value = if self.value >= rhs.value { 1. } else { 0. };
        let mut cons = Self::from_number(value, format!("{} >= {}", self.text, rhs.text));
        cons.boolean = true;
        cons.dice = self.dice + rhs.dice;
        cons
    }
    pub fn eq(self, rhs: Self) -> Self {
        let value = if self.value == rhs.value { 1. } else { 0. };
        let mut cons = Self::from_number(value, format!("{} = {}", self.text, rhs.text));
        cons.boolean = true;
        cons.dice = self.dice + rhs.dice;
        cons
    }
    pub fn percent(mut self) -> Self {
        self.value /= 100.0;
        self.values = vec![self.value];
        self.text += "%";
        self
    }
    pub fn ceil(mut self) -> Self {
        self.value = self.value.ceil();
        self.values = vec![self.value];
        self.text = format!("^{}", self.text);
        self
    }
    pub fn round(mut self) -> Self {
        self.value = self.value.round();
        self.values = vec![self.value];
        self.text = format!("~{}", self.text);
        self
    }
    pub fn floor(mut self) -> Self {
        self.value = self.value.floor();
        self.values = vec![self.value];
        self.text = format!("_{}", self.text);
        self
    }
}

impl ToString for RogCons {
    fn to_string(&self) -> String {
        if self.boolean {
            format!(
                "` {} ` ⟵ {}",
                if self.value == 0. {
                    "Falha!"
                } else {
                    "Sucesso!"
                },
                self.text
            )
        } else {
            format!("` {} ` ⟵ {}", self.value, self.text)
        }
    }
}

impl std::ops::Add for RogCons {
    type Output = RogCons;
    fn add(self, rhs: Self) -> Self::Output {
        let mut cons = Self::from_number(
            self.value + rhs.value,
            format!("{} + {}", self.text, rhs.text),
        );
        cons.dice = self.dice + rhs.dice;
        cons
    }
}
impl std::ops::Sub for RogCons {
    type Output = RogCons;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut cons = Self::from_number(
            self.value - rhs.value,
            format!("{} - {}", self.text, rhs.text),
        );
        cons.dice = self.dice + rhs.dice;
        cons
    }
}
impl std::ops::Mul for RogCons {
    type Output = RogCons;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut cons = Self::from_number(
            self.value * rhs.value,
            format!("{} * {}", self.text, rhs.text),
        );
        cons.dice = self.dice + rhs.dice;
        cons
    }
}
impl std::ops::Div for RogCons {
    type Output = RogCons;
    fn div(self, rhs: Self) -> Self::Output {
        let mut cons = Self::from_number(
            self.value / rhs.value,
            format!("{} / {}", self.text, rhs.text),
        );
        cons.dice = self.dice + rhs.dice;
        cons
    }
}
impl std::ops::BitAnd for RogCons {
    type Output = RogCons;
    fn bitand(self, rhs: Self) -> Self::Output {
        let a = !(self.value == 0.);
        let b = !(rhs.value == 0.);
        let mut cons = RogCons::from_number(
            if a && b { 1. } else { 0. },
            format!("{} & {}", self.text, rhs.text),
        );
        cons.boolean = true;
        cons.dice = self.dice + rhs.dice;
        cons
    }
}
impl std::ops::BitOr for RogCons {
    type Output = RogCons;
    fn bitor(self, rhs: Self) -> Self::Output {
        let a = !(self.value == 0.);
        let b = !(rhs.value == 0.);
        let mut cons = RogCons::from_number(
            if a || b { 1. } else { 0. },
            format!("{} | {}", self.text, rhs.text),
        );
        cons.boolean = true;
        cons.dice = self.dice + rhs.dice;
        cons
    }
}
impl std::ops::Shl for RogCons {
    type Output = RogCons;
    fn shl(self, rhs: Self) -> Self::Output {
        let value = self.values.into_iter().filter(|v| v <= &rhs.value).count();
        let mut cons = RogCons::from_number(value as f64, format!("{} << {}", self.text, rhs.text));
        cons.dice = self.dice + rhs.dice;
        cons
    }
}
impl std::ops::Shr for RogCons {
    type Output = RogCons;
    fn shr(self, rhs: Self) -> Self::Output {
        let value = self.values.into_iter().filter(|v| v >= &rhs.value).count();
        let mut cons = RogCons::from_number(value as f64, format!("{} >> {}", self.text, rhs.text));
        cons.dice = self.dice + rhs.dice;
        cons
    }
}
impl std::ops::Neg for RogCons {
    type Output = RogCons;
    fn neg(self) -> Self::Output {
        let mut cons = RogCons::from_number(-self.value, format!("-{}", self.text));
        cons.dice = self.dice;
        cons
    }
}
impl std::ops::Not for RogCons {
    type Output = RogCons;
    fn not(self) -> Self::Output {
        let value = if self.value != 0. { 0. } else { 1. };
        let mut cons = Self::from_number(value, format!("!{}", self.text));
        cons.dice = self.dice;
        cons.boolean = true;
        cons
    }
}
