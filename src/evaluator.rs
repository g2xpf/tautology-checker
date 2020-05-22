use super::parser::*;
use std::collections::{HashMap, HashSet};
use std::fmt;

struct VarEnv {
    bit: u32,
    assoc: [usize; 26],
    len: u32,
}

type Vars = HashSet<char>;
pub type BitPairs = HashMap<char, u32>;

#[derive(Debug)]
pub enum EvalResult {
    Tautology,
    NotTautology(BitPairs),
}

impl VarEnv {
    fn new(e: &Expr) -> Self {
        let vars = Self::vars(e);
        let mut env = VarEnv {
            bit: 0,
            assoc: [27; 26],
            len: vars.len() as u32,
        };
        for (i, var) in vars.iter().enumerate() {
            env.assoc[(*var as u8 - b'a') as usize] = i;
        }
        env
    }

    fn vars(e: &Expr) -> Vars {
        let mut bh = Vars::new();
        match e {
            Expr::Sole(e) => {
                bh.insert(e.0);
            }
            Expr::UnOp(UnOpExpr { expr, .. }) => bh.extend(Self::vars(expr)),
            Expr::BinOp(BinOpExpr { lhs, rhs, .. }) => {
                bh.extend(Self::vars(lhs));
                bh.extend(Self::vars(rhs));
            }
        }
        bh
    }

    fn iter(&self) -> Iter {
        Iter {
            len: self.len,
            bit: self.bit,
            assoc: &self.assoc,
        }
    }
}

struct Iter<'a> {
    len: u32,
    bit: u32,
    assoc: &'a [usize; 26],
}

struct VarEnvMap<'a> {
    assoc: &'a [usize; 26],
    bit: u32,
}

impl fmt::Debug for VarEnvMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_bit_pairs())
    }
}

impl<'a> VarEnvMap<'a> {
    fn new(assoc: &'a [usize; 26], bit: u32) -> Self {
        VarEnvMap { assoc, bit }
    }

    fn get(&self, c: char) -> bool {
        assert!(('a'..='z').contains(&c));
        self.bit >> (self.assoc[(c as u8 - b'a') as usize]) == 1
    }

    fn to_bit_pairs(&self) -> BitPairs {
        self.assoc
            .iter()
            .enumerate()
            .filter(|(_i, v)| **v != 27)
            .map(|(i, v)| ((i as u8 + b'a') as char, (self.bit >> v) & 1))
            .collect()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = VarEnvMap<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bit < 1 << self.len {
            self.bit += 1;
            Some(VarEnvMap::new(self.assoc, self.bit - 1))
        } else {
            None
        }
    }
}

fn imp(lhs: bool, rhs: bool) -> bool {
    !lhs || rhs
}

pub fn eval(parser: &Parser) -> EvalResult {
    let env = &parser.0;
    let var_env = VarEnv::new(env);
    for map in var_env.iter() {
        if !evaluate(env, &map) {
            return EvalResult::NotTautology(map.to_bit_pairs());
        }
    }
    EvalResult::Tautology
}

fn evaluate(e: &Expr, env: &VarEnvMap<'_>) -> bool {
    match e {
        Expr::Sole(e) => env.get(e.0),
        Expr::BinOp(BinOpExpr { lhs, op, rhs }) => {
            let l = evaluate(lhs, env);
            let r = evaluate(rhs, env);
            match op {
                BinOp::And => l && r,
                BinOp::Or => l || r,
                BinOp::Imp => imp(l, r),
            }
        }
        Expr::UnOp(UnOpExpr { expr, .. }) => !evaluate(expr, env),
    }
}
