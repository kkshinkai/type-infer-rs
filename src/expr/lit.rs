// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(i32),
    Float(f64),
    Bool(bool),
    String(String),
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lit::Int(int) => write!(f, "{}", int),
            Lit::Float(float) => write!(f, "{}", float),
            Lit::Bool(bool) => write!(f, "{}", bool),
            Lit::String(string) => write!(f, "{}", string),
        }
    }
}

pub trait ToLit {
    fn to_lit(self) -> Lit;
}

impl ToLit for i32 {
    fn to_lit(self) -> Lit {
        Lit::Int(self)
    }
}

impl ToLit for f64 {
    fn to_lit(self) -> Lit {
        Lit::Float(self)
    }
}

impl ToLit for bool {
    fn to_lit(self) -> Lit {
        Lit::Bool(self)
    }
}

impl ToLit for &str {
    fn to_lit(self) -> Lit {
        Lit::String(self.to_string())
    }
}
