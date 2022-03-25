// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use graphics::{Game, Program};
use zstring::zstr;

fn main() {
    let program =
        Program::init((800, 600), zstr!("Breakout"), true).expect("Failed to initialize program");
    program.execute();
}
