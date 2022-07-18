// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExprId {
    private: u32,
}

pub const DUMMY_NODE_ID: ExprId = ExprId { private: 0 };

impl ExprId {
    pub fn from_u32(id: u32) -> ExprId {
        debug_assert_ne!(id, 0, "0 is not a valid expression id");
        ExprId { private: id }
    }

    pub fn dummy() -> ExprId {
        DUMMY_NODE_ID
    }
}
