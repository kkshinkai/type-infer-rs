// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

#[derive(Debug, Clone, PartialEq)]
pub enum TyError {
    // FIXME: Remove `Unknown` error later.

    /// Represents an ad-hoc error message.
    Unknown(String),
}

/// Type inference result for reporting type errors.
pub type TyResult<T> = Result<T, TyError>;
