// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    path::Path,
    sync::{Mutex, MutexGuard},
};

use once_cell::sync::{Lazy, OnceCell};

use super::{
    shader::{Shader, ShaderCompileArgs},
    texture::{Texture, TextureOptions},
};

mod detail {
    pub(in crate::breakout::resman) struct DontCreateMe;
}

// pub type Lock<T> = RefCell<T>;
// pub type Reader<'r, T> = Ref<'r, T>;
// pub type Writer<'w, T> = RefMut<'w, T>;

pub type Lock<T> = Mutex<T>;
pub type Reader<'r, T> = MutexGuard<'r, T>;
pub type Writer<'w, T> = MutexGuard<'w, T>;

/// Header: https://learnopengl.com/code_viewer_gh.php?code=src/7.in_practice/3.2d_game/0.full_source/resource_manager.h
/// Source: https://learnopengl.com/code_viewer_gh.php?code=src/7.in_practice/3.2d_game/0.full_source/resource_manager.cpp
pub struct ResourceManager {
    _guard: detail::DontCreateMe,
    shaders: Lock<HashMap<String, Shader>>,
    textures: Lock<HashMap<String, Texture>>,
}

impl ResourceManager {
    pub fn instance() -> &'static ResourceManager {
        static SINGLE: OnceCell<ResourceManager> = OnceCell::new();
        SINGLE.get_or_init(ResourceManager::initialize)
    }

    pub fn load_shader<S: AsRef<str>>(
        &self,
        gl: &glitz::GlFns,
        name: S,
        args: &ShaderCompileArgs,
    ) -> Option<Shader> {
        let name_s = name.as_ref().to_string();
        println!("load_shader called with name {}", &name_s);

        println!("calling load_shader_internal");
        let loaded = match Self::load_shader_internal(gl, args) {
            Some(shader) => shader,
            None => return None,
        };
        println!("load_shader_internal success");

        if let Some(old) = self.shaders.lock().ok()?.insert(name_s, loaded) {
            eprintln!(
                "Overwriting shader {}, old id = {} new id = {}",
                name.as_ref(),
                old.id(),
                loaded.id()
            );
            if old.id() != loaded.id() {
                /// TODO: The tutorial does NOT do when a shader is loaded (because it does not check if it exists first), but it does call this for each member of each map in "Clear"
                gl.DeleteProgram(old.id());
            }
        }

        Some(loaded)
    }

    pub fn get_shader(&self, gl: &glitz::GlFns, name: &str) -> Option<Shader> {
        self.shaders.lock().ok()?.get(&name.to_string()).copied()
    }

    pub fn load_texture<S: AsRef<str>, P: AsRef<Path>>(
        &self,
        gl: &glitz::GlFns,
        name: S,
        file: P,
        alpha: bool,
    ) -> Option<Texture> {
        println!("load_texture called with name {}", &name.as_ref());
        println!("calling load_texture_internal");
        let loaded = match Self::load_texture_internal(gl, file, alpha) {
            Some(tex) => tex,
            None => return None,
        };
        println!("load_texture_internal complete");

        if let Some(old) = self
            .textures
            .lock()
            .ok()?
            .insert(name.as_ref().to_string(), loaded)
        {
            eprintln!(
                "Overwriting texture {}, old id = {} new id = {}",
                name.as_ref(),
                old.id(),
                loaded.id()
            );

            if old.id() != loaded.id() {
                let id = old.id();
                // let arr = [old.id()];
                /// TODO: The tutorial does NOT do when a texture is loaded (because it does not check if it exists first), but it does call this for each member of each map in "Clear"
                unsafe {
                    gl.DeleteTextures(1, &id);
                }
            }
        }

        Some(loaded)
    }

    pub fn get_texture(&self, gl: &glitz::GlFns, name: &str) -> Option<Texture> {
        self.textures.lock().ok()?.get(&name.to_string()).copied()
    }

    pub fn dispose_all(&self, gl: &glitz::GlFns) {
        if let Ok(mut shaders) = self.shaders.lock() {
            for (_, shader) in shaders.drain() {
                gl.DeleteProgram(shader.id());
            }
        } else {
            eprintln!("Failed to lock shaders");
        }

        if let Ok(mut textures) = self.textures.lock() {
            let ids = textures
                .drain()
                .map(|(_, tex)| tex.id())
                .collect::<Vec<_>>();
            unsafe {
                gl.DeleteTextures(ids.len() as _, ids.as_ptr());
            }
        } else {
            eprintln!("Failed to lock shaders");
        }
    }

    fn initialize() -> Self {
        Self {
            _guard: detail::DontCreateMe,
            shaders: Default::default(),
            textures: Default::default(),
        }
    }

    fn load_shader_internal(gl: &glitz::GlFns, args: &ShaderCompileArgs) -> Option<Shader> {
        println!("load_shader_internal called");
        if !args.is_cstr_valid() {
            eprintln!("Shaders provided to load_shader_internal are not valid CStrings");
            return None;
        }

        println!("creating shader");
        let mut shader = Shader::new();
        println!("compiling shader");
        shader.compile(gl, args).then_some(shader)
    }

    fn load_texture_internal<P: AsRef<Path>>(
        gl: &glitz::GlFns,
        file: P,
        alpha: bool,
    ) -> Option<Texture> {
        println!("load_texture_internal called");
        let file = file.as_ref();
        if !file.exists() {
            eprintln!("Texture file {} does not exist", file.display());
            return None;
        }

        println!("loading texture from file");
        let image = match stb_image::image::load(file) {
            stb_image::image::LoadResult::Error(err) => {
                eprintln!("Error loading image from file {}: {}", file.display(), err);
                return None;
            }
            stb_image::image::LoadResult::ImageU8(img) => img,
            stb_image::image::LoadResult::ImageF32(_) => {
                eprintln!("F32 textures are not supported!");
                return None;
            }
        };
        println!("successfully loaded image");

        println!("creating texture");
        let mut tex = if alpha {
            Texture::with_alpha(gl)
        } else {
            Texture::new(gl)
        };

        println!("generating texture");
        tex.generate(
            gl,
            (image.width as u32, image.height as u32).into(),
            &image.data,
        );

        Some(tex)
    }
}
