// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(i32),
    Bool(bool),
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lit::Int(int) => write!(f, "{}", int),
            Lit::Bool(bool) => write!(f, "{}", bool),
        }
    }
}
