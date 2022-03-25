// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use beryllium::{
    event::Event,
    gl_window::{GlAttr, GlContextFlags, GlProfile, GlWindow},
    init::{InitFlags, Sdl},
    window::WindowFlags,
    SdlResult,
};

use std::{ptr, str};
use zstring::{zstr, ZStr};

use crate::{breakout::game::InputStatus, Game};

pub struct Program {
    sdl: Sdl,
    gl_win: GlWindow,
    gl: glitz::GlFns,
    win_size: (u16, u16),
}

impl Program {
    pub fn init(size: (u16, u16), title: ZStr<'_>, debug_cb: bool) -> SdlResult<Self> {
        let sdl = Sdl::init(InitFlags::EVERYTHING)?;
        sdl.allow_drop_events(true);

        const FLAGS: i32 = if cfg!(debug_assertions) {
            GlContextFlags::FORWARD_COMPATIBLE.as_i32() | GlContextFlags::DEBUG.as_i32()
        } else {
            GlContextFlags::FORWARD_COMPATIBLE.as_i32()
        };
        sdl.gl_set_attribute(GlAttr::MajorVersion, 3)?;
        sdl.gl_set_attribute(GlAttr::MinorVersion, 3)?;
        sdl.gl_set_attribute(GlAttr::Profile, GlProfile::Core as _)?;
        sdl.gl_set_attribute(GlAttr::Flags, FLAGS)?;

        let gl_win = sdl.create_gl_window(
            title,
            None,
            (size.0 as i32, size.1 as i32),
            WindowFlags::ALLOW_HIGHDPI,
        )?;
        gl_win.set_swap_interval(1)?;

        let gl = unsafe { glitz::GlFns::from_loader(&|zs| gl_win.get_proc_address(zs)).unwrap() };
        if debug_cb && gl_win.is_extension_supported(zstr!("GL_KHR_debug")) {
            println!("Activating the debug callback...");
            unsafe { gl.DebugMessageCallback(Some(glitz::println_gl_debug_callback), ptr::null()) };
        }

        gl.Enable(glitz::GL_BLEND);
        // gl.Enable(glitz::GL_DEPTH_TEST);
        gl.BlendFunc(glitz::GL_SRC_ALPHA, glitz::GL_ONE_MINUS_SRC_ALPHA);

        gl.ClearColor(0.2, 0.6, 0.8, 1.0);

        Ok(Self {
            sdl,
            gl_win,
            gl,
            win_size: size,
        })
    }

    pub fn execute(&self) {
        let mut game = Game::init(&self.gl, self.win_size);
        let mut input_status = InputStatus::Continue;
        let mut last = 0.0;
        let mut delta = 0.0;
        while input_status == InputStatus::Continue {
            let current_ticks = self.sdl.get_ticks();
            delta = current_ticks as f32 - last;
            last = current_ticks as f32;
            input_status = game.handle_input(&self.gl, &self.sdl, delta);
            game.update(&self.gl, &self.sdl, delta);

            self.gl.ClearColor(0.0, 0.0, 0.0, 0.0);
            self.gl.Clear(glitz::GL_COLOR_BUFFER_BIT);
            game.render(&self.gl, &self.sdl, &self.gl_win);
            self.gl_win.swap_backbuffer();
        }

        game.before_close(&self.gl);
    }
}
