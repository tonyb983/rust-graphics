// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![feature(
    bool_to_option,
    cstr_from_bytes_until_nul,
    box_patterns,
    box_syntax,
    ptr_const_cast,
    ptr_metadata
)]
#![allow(dead_code, unused)]

mod breakout;
mod ui;

pub use breakout::{Game, Program};
pub use ui::execute;
