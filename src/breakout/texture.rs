// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::types::{Vec2U, Zero};

/// Header: https://learnopengl.com/code_viewer_gh.php?code=src/7.in_practice/3.2d_game/0.full_source/texture.h
/// Source: https://learnopengl.com/code_viewer_gh.php?code=src/7.in_practice/3.2d_game/0.full_source/texture.cpp
#[derive(Debug, Clone, Copy)]
pub struct Texture {
    id: u32,
    size: Vec2U,
    opts: TextureOptions,
    is_bound: bool,
}

impl PartialEq<Self> for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureOptions {
    pub(crate) internal_format: u32,
    pub(crate) image_format: u32,
    pub(crate) wrap_s: u32,
    pub(crate) wrap_t: u32,
    pub(crate) min_filter: u32,
    pub(crate) max_filter: u32,
}

impl Default for TextureOptions {
    fn default() -> Self {
        Self {
            internal_format: glitz::GL_RGB,
            image_format: glitz::GL_RGB,
            wrap_s: glitz::GL_REPEAT,
            wrap_t: glitz::GL_REPEAT,
            min_filter: glitz::GL_LINEAR,
            max_filter: glitz::GL_LINEAR,
        }
    }
}

impl Texture {
    pub fn new(gl: &glitz::GlFns) -> Self {
        Self::with_options(gl, Default::default())
    }

    pub fn with_alpha(gl: &glitz::GlFns) -> Self {
        Self::with_options(
            gl,
            TextureOptions {
                internal_format: glitz::GL_RGBA,
                image_format: glitz::GL_RGBA,
                ..Default::default()
            },
        )
    }

    pub fn with_options(gl: &glitz::GlFns, opts: TextureOptions) -> Self {
        let mut id = 0;
        unsafe {
            gl.GenTextures(1, &mut id);
        }
        Self {
            id,
            size: Vec2U::zero(),
            opts,
            is_bound: false,
        }
    }

    pub fn generate(&mut self, gl: &glitz::GlFns, size: Vec2U, data: &[u8]) {
        use glitz::{
            GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_WRAP_S,
            GL_TEXTURE_WRAP_T, GL_UNSIGNED_BYTE,
        };
        self.size = size;
        // Create texture
        self.bind(gl);
        unsafe {
            gl.TexImage2D(
                GL_TEXTURE_2D,
                0,
                self.internal_format() as i32,
                self.width() as i32,
                self.height() as i32,
                0,
                self.image_format(),
                GL_UNSIGNED_BYTE,
                data.as_ptr().cast(),
            );
        }
        // Set texture params
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, self.wrap_s() as i32);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, self.wrap_t() as i32);
        gl.TexParameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_MIN_FILTER,
            self.min_filter() as i32,
        );
        gl.TexParameteri(
            GL_TEXTURE_2D,
            GL_TEXTURE_MAG_FILTER,
            self.max_filter() as i32,
        );
        // Unbind texture
        self.unbind(gl);
    }

    pub fn bind(&self, gl: &glitz::GlFns) {
        gl.BindTexture(glitz::GL_TEXTURE_2D, self.id);
        // if !self.is_bound {
        //     gl.BindTexture(glitz::GL_TEXTURE_2D, self.id);
        //     self.is_bound = true;
        // } else {
        //     eprintln!(
        //         "bind called but texture is already bound (id = {})",
        //         self.id
        //     );
        // }
    }

    pub fn unbind(&self, gl: &glitz::GlFns) {
        gl.BindTexture(glitz::GL_TEXTURE_2D, 0);
        // if self.is_bound {
        //     gl.BindTexture(glitz::GL_TEXTURE_2D, 0);
        //     self.is_bound = false;
        // } else {
        //     eprintln!(
        //         "unbind called but texture is not currently bound (id = {})",
        //         self.id
        //     );
        // }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn width(&self) -> u32 {
        self.size.x
    }

    pub fn height(&self) -> u32 {
        self.size.y
    }

    pub fn wrap_s(&self) -> u32 {
        self.opts.wrap_s
    }

    pub fn wrap_t(&self) -> u32 {
        self.opts.wrap_t
    }

    pub fn min_filter(&self) -> u32 {
        self.opts.min_filter
    }

    pub fn max_filter(&self) -> u32 {
        self.opts.max_filter
    }

    pub fn internal_format(&self) -> u32 {
        self.opts.internal_format
    }

    pub fn image_format(&self) -> u32 {
        self.opts.image_format
    }

    pub fn modify_options(&mut self, mut f: impl FnOnce(&TextureOptions) -> TextureOptions) {
        self.opts = f(&self.opts);
    }

    pub fn set_options(&mut self, opts: TextureOptions) {
        self.opts = opts;
    }
}
