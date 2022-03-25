// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{mem, ptr};

use crate::breakout::types::Mat4F;

use super::{
    shader::Shader,
    texture::Texture,
    types::{Vec2F, Vec3F},
};

#[derive(Debug, Clone)]
pub struct DrawSpriteArgs<'tex> {
    texture: &'tex Texture,
    pos: Vec2F,
    size: Vec2F,
    rotate: f32,
    color: Vec3F,
}

impl<'tex> DrawSpriteArgs<'tex> {
    pub fn new(texture: &'tex Texture, pos: Vec2F, size: Vec2F, rotate: f32, color: Vec3F) -> Self {
        DrawSpriteArgs {
            texture,
            pos,
            size,
            rotate,
            color,
        }
    }

    pub fn texture(&self) -> &'tex Texture {
        self.texture
    }

    pub fn pos(&self) -> Vec2F {
        self.pos
    }

    pub fn size(&self) -> Vec2F {
        self.size
    }

    pub fn rotate(&self) -> f32 {
        self.rotate
    }

    pub fn color(&self) -> Vec3F {
        self.color
    }
}

pub struct SpriteRenderer {
    shader: Shader,
    quad_vao: u32,
    quad_vbo: u32,
}

impl SpriteRenderer {
    pub fn new(gl: &glitz::GlFns, shader: &Shader) -> Self {
        let mut this = Self {
            shader: *shader,
            quad_vao: 0,
            quad_vbo: 0,
        };
        this.init_render_data(gl);
        this
    }

    pub fn draw_sprite(&self, gl: &glitz::GlFns, args: &DrawSpriteArgs) {
        use super::util;
        use cgmath::{vec3, SquareMatrix};

        self.shader.set_main(gl);
        let mut model = Mat4F::identity();
        let translate1 = args.pos().extend(0.0);
        let translate2 = (args.size() * 0.5).extend(0.0);
        let rotate = vec3(0.0, 0.0, 1.0);
        let scale = args.size().extend(1.0);

        util::mat4_translate_in(&mut model, &translate1);
        util::mat4_translate_in(&mut model, &translate2);
        util::mat4_rotate_in(&mut model, args.rotate().to_radians(), &rotate);
        util::mat4_scale_in(&mut model, &scale);

        self.shader.set_matrix4f_from(gl, "model", model, false);
        self.shader
            .set_vector3f(gl, "spriteColor", args.color(), false);

        gl.ActiveTexture(glitz::GL_TEXTURE0);
        args.texture().bind(gl);

        gl.BindVertexArray(self.quad_vao);
        unsafe {
            gl.DrawArrays(glitz::GL_TRIANGLES, 0, 16);
        }
        gl.BindVertexArray(0);
    }

    pub fn uninit(&mut self, gl: &glitz::GlFns) {
        unsafe {
            gl.DeleteVertexArrays(1, &self.quad_vao);
            gl.DeleteBuffers(1, &self.quad_vbo);
        }

        self.quad_vao = 0;
        self.quad_vbo = 0;
    }

    fn init_render_data(&mut self, gl: &glitz::GlFns) {
        use glitz::{GL_ARRAY_BUFFER, GL_FALSE, GL_FLOAT, GL_STATIC_DRAW};
        let mut vao = 0u32;
        let mut vbo = 0u32;

        let vertices = make_vertices();

        unsafe {
            gl.GenVertexArrays(1, &mut vao); // unsafe
            gl.GenBuffers(1, &mut vbo); // unsafe

            gl.BindBuffer(GL_ARRAY_BUFFER, vbo); // safe
            gl.BufferData(
                GL_ARRAY_BUFFER,
                vertices.len() as isize,
                vertices.as_ptr().cast(),
                GL_STATIC_DRAW,
            ); // unsafe

            gl.BindVertexArray(vao); // safe
            gl.EnableVertexAttribArray(0); // safe
            gl.VertexAttribPointer(
                0,
                4,
                GL_FLOAT,
                GL_FALSE as u8,
                4 * mem::size_of::<f32>() as i32,
                ptr::null(),
            ); // unsafe
            gl.BindBuffer(GL_ARRAY_BUFFER, 0); // safe
            gl.BindVertexArray(0); // safe
        }

        self.quad_vao = vao;
        self.quad_vbo = vbo;
    }
}

#[rustfmt::skip]
fn make_vertices() -> [f32; 24] {
    [
        0.0f32, 1.0f32, 0.0f32, 1.0f32, 
        1.0f32, 0.0f32, 1.0f32, 0.0f32, 
        0.0f32, 0.0f32, 0.0f32, 0.0f32, 
        0.0f32, 1.0f32, 0.0f32, 1.0f32, 
        1.0f32, 1.0f32, 1.0f32, 1.0f32, 
        1.0f32, 0.0f32, 1.0f32, 0.0f32,
    ]
}
