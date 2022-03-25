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

use crate::breakout::{
    render::{DrawSpriteArgs, SpriteRenderer},
    resman::ResourceManager,
    shader::ShaderCompileArgs,
    texture,
    types::Mat4F,
};

pub enum State {
    Active,
    Menu,
    Win,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputStatus {
    Continue,
    Quit,
}

pub struct Game {
    state: State,
    keys: [bool; 1024],
    size: (u16, u16),
    renderer: SpriteRenderer,
}

impl Game {
    pub fn init(gl: &glitz::GlFns, window_size: (u16, u16)) -> Self {
        println!("game init starting");
        let sprite_shader_args = ShaderCompileArgs::from_files::<_, _, &str>(
            "C:\\Tony\\Code\\Rust\\graphics\\assets\\shaders\\sprite\\sprite.vs",
            "C:\\Tony\\Code\\Rust\\graphics\\assets\\shaders\\sprite\\sprite.frag",
            None,
        )
        .expect("Unable to load sprite shaders from files.");
        println!("sprite shader args: {:?}", sprite_shader_args);

        let sprite_shader =
            match ResourceManager::instance().load_shader(gl, "sprite", &sprite_shader_args) {
                Some(sh) => sh,
                None => panic!("Unable to load sprite shader."),
            };
        println!("Sprite shader loaded");

        let projection: Mat4F = cgmath::ortho(
            0.0,
            window_size.0 as f32,
            window_size.1 as f32,
            0.0,
            -1.0,
            1.0,
        );
        println!("setting image integer");
        sprite_shader.set_main(gl);
        sprite_shader.set_integer(gl, "image", 0, true);
        println!("setting projection matrix");
        sprite_shader.set_matrix4f_from(gl, "projection", projection, false);
        let renderer = SpriteRenderer::new(gl, &sprite_shader);
        println!("loading awesomeface");
        match ResourceManager::instance().load_texture(
            gl,
            "face",
            "C:\\Tony\\Code\\Rust\\graphics\\assets\\textures\\face.png",
            true,
        ) {
            Some(tex) => println!("successfully loaded awesomeface: {:?}", tex),
            None => panic!("failed to load awesomeface"),
        }

        println!("game init complete");
        Self {
            state: State::Active,
            keys: [false; 1024],
            size: window_size,
            renderer,
        }
    }

    pub fn execute(&self, gl: &glitz::GlFns, sdl: &Sdl, gl_win: &GlWindow) {
        let mut input_status = InputStatus::Continue;
        let mut last = 0.0;
        let mut delta = 0.0;
        while input_status == InputStatus::Continue {
            let current_ticks = sdl.get_ticks();
            delta = current_ticks as f32 - last;
            last = current_ticks as f32;
            input_status = self.handle_input(gl, sdl, delta);
            self.update(gl, sdl, delta);

            gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            gl.Clear(glitz::GL_COLOR_BUFFER_BIT);
            self.render(gl, sdl, gl_win);
            gl_win.swap_backbuffer();
        }
    }

    pub fn handle_input(&self, gl: &glitz::GlFns, sdl: &Sdl, delta: f32) -> InputStatus {
        while let Some(e) = sdl.poll_event() {
            match e {
                Event::Quit => return InputStatus::Quit,
                Event::MouseMotion { .. } => (),
                Event::Keyboard { .. } => (),
                Event::TextInput { text, .. } => {
                    println!("TextInput: {:?}", str::from_utf8(&text));
                }
                other => println!("Event: {:?}", other),
            }
        }

        InputStatus::Continue
    }

    pub fn update(&self, gl: &glitz::GlFns, sdl: &Sdl, delta: f32) {}

    pub fn render(&self, gl: &glitz::GlFns, sdl: &Sdl, gl_win: &GlWindow) {
        use super::types::{vec2, vec3};

        let face = match ResourceManager::instance().get_texture(gl, "face") {
            Some(tex) => tex,
            None => panic!("Unable to load awesomeface"),
        };
        let args = DrawSpriteArgs::new(
            &face,
            vec2(200.0, 200.0),
            vec2(300.0, 400.0),
            45.0,
            vec3(0.0, 1.0, 0.0),
        );

        self.renderer.draw_sprite(gl, &args);
    }

    pub fn before_close(&mut self, gl: &glitz::GlFns) {
        ResourceManager::instance().dispose_all(gl);
        self.renderer.uninit(gl);
    }
}
