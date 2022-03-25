// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub type Vec2 = (f32, f32);
pub type Vec3 = (f32, f32, f32);
pub type Vec4 = (f32, f32, f32, f32);
pub type Mat4 = [[f32; 4]; 4];

mod game;
mod program;
mod render;
mod resman;
mod shader;
mod texture;

mod types {
    use cgmath::{Matrix4, Quaternion as QuaternionT, Vector1, Vector2, Vector3, Vector4};

    pub use cgmath::prelude::*;
    pub use cgmath::{vec1, vec2, vec3, vec4, Matrix};

    pub type Vec2F = Vector2<f32>;
    pub type Vec2I = Vector2<i32>;
    pub type Vec2U = Vector2<u32>;
    pub type Vec3F = Vector3<f32>;
    pub type Vec3I = Vector3<i32>;
    pub type Vec3U = Vector3<u32>;
    pub type Vec4F = Vector4<f32>;
    pub type Vec4I = Vector4<i32>;
    pub type Vec4U = Vector4<u32>;
    pub type Mat4F = Matrix4<f32>;
    pub type Mat4I = Matrix4<i32>;
    pub type Mat4U = Matrix4<u32>;
    pub type Quaternion = QuaternionT<f32>;

    pub fn mat4_to_array<T: Copy>(mat: &Matrix4<T>) -> [T; 16] {
        [
            mat[0][0], mat[0][1], mat[0][2], mat[0][3], mat[1][0], mat[1][1], mat[1][2], mat[1][3],
            mat[2][0], mat[2][1], mat[2][2], mat[2][3], mat[3][0], mat[3][1], mat[3][2], mat[3][3],
        ]
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

        #[test]
        fn vec2f() {
            let v2 = Vec2F::new(1.0, 2.0);
            println!("v2 = {:?}", v2);
            println!("sizeof 1f32 = {:?}", std::mem::size_of_val(&1f32));
            println!("sizeof v2.x = {:?}", std::mem::size_of_val(&v2.x));
            println!("sizeof v2 = {:?}", std::mem::size_of_val(&v2));
            let x_ptr: *const f32 = &v2.x;
            let y_ptr: *const f32 = &v2.y;
            let add_ptr: *const f32 = unsafe { x_ptr.add(1) };
            let ptr = v2.as_ptr();
            println!(" ptr = {:?}", ptr);
            println!("x_ptr = {:?}", x_ptr);
            println!("y_ptr = {:?}", y_ptr);
            println!("+_ptr = {:?}", add_ptr);
        }

        #[test]
        fn mat4f() {
            let mat = Mat4F::new(
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            );

            let values = mat4_to_array(&mat);
            println!(" mat4f = {:?}", mat);
            println!("imat4f = {:?}", mat.invert().unwrap());
            println!("values = {:?}", values);
        }
    }
}

mod util {
    use super::types::{Mat4F, Vec3F};
    pub fn get_program_info_log(gl: &glitz::GlFns, id: u32, debug: bool) -> Result<String, String> {
        let mut len = -1;
        unsafe {
            gl.GetProgramiv(id, glitz::GL_INFO_LOG_LENGTH, &mut len);
        }
        if debug {
            println!(
                "get_program_info_log - GetProgramiv returned len of {}",
                len
            );
        }

        if len < 0 {
            return Err("get_program_info_log - could not get info log length".to_string());
        }

        if len == 0 {
            return Err("get_program_info_log - no info log to get".to_string());
        }

        let mut buf = vec![0u8; len as usize];
        let mut written = 0;
        unsafe {
            gl.GetProgramInfoLog(id, len, &mut written, buf.as_mut_ptr().cast());
        }
        if debug {
            println!(
                "get_program_info_log - GetProgramInfoLog returned written of {}",
                written
            );
            println!(
                "get_program_info_log - GetProgramInfoLog returned raw data of {:?}",
                buf
            );
        }

        if written != len {
            println!(
                "get_program_info_log - WARNING written != len, expected len of {} but written is {}",
                len, written
            );
        }

        let s = String::from_utf8_lossy(buf.as_slice()).into_owned();

        Ok(s)
    }

    pub fn get_shader_info_log(gl: &glitz::GlFns, id: u32, debug: bool) -> Result<String, String> {
        let mut len = -1;
        unsafe {
            gl.GetShaderiv(id, glitz::GL_INFO_LOG_LENGTH, &mut len);
        }
        if debug {
            println!("get_shader_info_log- GetShaderiv returned len of {}", len);
        }

        if len < 0 {
            return Err("get_shader_info_log - could not get info log length".to_string());
        }

        if len == 0 {
            return Err("get_shader_info_log - no info log to get".to_string());
        }

        let mut buf = vec![0u8; (len) as usize];
        let mut written = -1;
        unsafe {
            gl.GetShaderInfoLog(id, len, &mut written, buf.as_mut_ptr().cast());
        }
        if debug {
            println!(
                "get_shader_info_log - GetShaderInfoLog returned written of {}",
                written
            );
            println!(
                "get_shader_info_log - GetShaderInfoLog returned raw data of {:?}",
                buf
            );
        }

        if written != len {
            println!(
                "get_shader_info_log - WARNING written != len, expected len of {} but written is {}",
                len, written
            );
        }

        Ok(String::from_utf8_lossy(buf.as_slice()).into_owned())
        // if written > 0 && written <= len {
        //     Ok(String::from_utf8_lossy(buf.as_slice()).into_owned())
        // } else {
        //     match std::ffi::CStr::from_bytes_until_nul(buf.as_slice())
        //         .map_err(|err| format!("ERROR: {}", err))
        //         .map(|cs| cs.to_string_lossy().into_owned())
        //     {
        //         Ok(s) => Ok(s),
        //         Err(err) => {
        //             println!("CStr::from_bytes_until_nul failed: {}", err);
        //             match buf.iter().position(|b| *b == 0) {
        //                 Some(idx) => Ok(String::from_utf8_lossy(&buf[0..idx]).into_owned()),
        //                 None => {
        //                     println!("Could not find null terminator");
        //                     Ok(String::from_utf8_lossy(buf.as_slice()).into_owned())
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    pub fn mat4_translate(matrix: &Mat4F, vec: &Vec3F) -> Mat4F {
        let mut result = *matrix;
        result.w = matrix.x * vec.x + matrix.y * vec.y + matrix[2] * vec.z + matrix.w;
        result
    }

    pub fn mat4_translate_in(matrix: &mut Mat4F, vec: &Vec3F) {
        matrix.w = matrix.x * vec.x + matrix.y * vec.y + matrix[2] * vec.z + matrix.w;
    }

    pub fn mat4_rotate(matrix: &Mat4F, angle: f32, vec: &Vec3F) -> Mat4F {
        use cgmath::{
            EuclideanSpace, InnerSpace, Matrix, SquareMatrix, Transform, VectorSpace, Zero,
        };

        let a = angle;
        let c = angle.cos();
        let s = angle.sin();

        let axis: Vec3F = vec.normalize();
        let temp: Vec3F = axis * (1.0 - c);
        let mut rotate = Mat4F::zero();
        rotate[0][0] = c + temp[0] * axis[0];
        rotate[0][1] = temp[0] * axis[1] + s * axis[2];
        rotate[0][2] = temp[0] * axis[2] - s * axis[1];

        rotate[1][0] = temp[1] * axis[0] - s * axis[2];
        rotate[1][1] = c + temp[1] * axis[1];
        rotate[1][2] = temp[1] * axis[2] + s * axis[0];

        rotate[2][0] = temp[2] * axis[0] + s * axis[1];
        rotate[2][1] = temp[2] * axis[1] - s * axis[0];
        rotate[2][2] = c + temp[2] * axis[2];

        let mut result = Mat4F::zero();
        result[0] = matrix[0] * rotate[0][0] + matrix[1] * rotate[0][1] + matrix[2] * rotate[0][2];
        result[1] = matrix[0] * rotate[1][0] + matrix[1] * rotate[1][1] + matrix[2] * rotate[1][2];
        result[2] = matrix[0] * rotate[2][0] + matrix[1] * rotate[2][1] + matrix[2] * rotate[2][2];
        result[3] = matrix[3];

        result
    }

    pub fn mat4_rotate_in(matrix: &mut Mat4F, angle: f32, vec: &Vec3F) {
        let result = mat4_rotate(matrix, angle, vec);
        *matrix = result;
    }

    pub fn mat4_scale(matrix: &Mat4F, vec: &Vec3F) -> Mat4F {
        let mut result = *matrix;
        result.x *= vec.x;
        result.y *= vec.y;
        result.z *= vec.z;
        result
    }

    pub fn mat4_scale_in(matrix: &mut Mat4F, vec: &Vec3F) {
        matrix.x *= vec.x;
        matrix.y *= vec.y;
        matrix.z *= vec.z;
    }
}

pub use game::Game;
pub use program::Program;
