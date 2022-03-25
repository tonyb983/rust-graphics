// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    ffi::{self, CStr, CString},
    os::raw::c_float,
    path::Path,
};

use zstring::{zstr, ZStr, ZString};

use super::types::{Mat4F, Matrix, Vec2F, Vec3F, Vec4F};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileType {
    Vertex,
    Fragment,
    Geometry,
    Program,
}

impl CompileType {
    pub fn is_program(&self) -> bool {
        *self == CompileType::Program
    }
}

impl std::fmt::Display for CompileType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CompileType::Vertex => write!(f, "Vertex"),
            CompileType::Fragment => write!(f, "Fragment"),
            CompileType::Geometry => write!(f, "Geometry"),
            CompileType::Program => write!(f, "Program"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShaderCompileArgs {
    vertex_source: String,
    fragment_source: String,
    geometry_source: Option<String>,
}

impl ShaderCompileArgs {
    pub fn from_sources<V: AsRef<str>, F: AsRef<str>, G: AsRef<str>>(
        vert_src: V,
        frag_src: F,
        geom_src: Option<G>,
    ) -> Self {
        Self {
            vertex_source: vert_src.as_ref().to_string(),
            fragment_source: frag_src.as_ref().to_string(),
            geometry_source: geom_src.map(|s| s.as_ref().to_string()),
        }
    }

    pub fn from_files<V: AsRef<Path>, F: AsRef<Path>, G: AsRef<Path>>(
        vert_file: V,
        frag_file: F,
        geom_file: Option<G>,
    ) -> std::io::Result<Self> {
        let vertex_source = std::fs::read_to_string(vert_file)?;
        let fragment_source = std::fs::read_to_string(frag_file)?;
        let geometry_source = if let Some(file) = geom_file {
            let text = std::fs::read_to_string(file)?;
            Some(text)
        } else {
            None
        };

        Ok(Self {
            vertex_source,
            fragment_source,
            geometry_source,
        })
    }

    pub fn has_geo(&self) -> bool {
        self.geometry_source.is_some()
    }

    pub fn is_cstr_valid(&self) -> bool {
        for byte in self.vertex_source.as_bytes() {
            if *byte == 0 {
                return false;
            }
        }

        for byte in self.fragment_source.as_bytes() {
            if *byte == 0 {
                return false;
            }
        }

        if let Some(s) = &self.geometry_source {
            for byte in s.as_bytes() {
                if *byte == 0 {
                    return false;
                }
            }
        }

        true
    }

    pub fn to_cstrings(&self) -> Option<(CString, CString, Option<CString>)> {
        if !self.is_cstr_valid() {
            return None;
        }

        let vertex_source = CString::new(self.vertex_source.clone().into_bytes()).unwrap();
        let fragment_source = CString::new(self.fragment_source.clone().into_bytes()).unwrap();
        let geometry_source = self
            .geometry_source
            .as_ref()
            .map(|s| CString::new(s.clone().into_bytes()).unwrap());

        Some((vertex_source, fragment_source, geometry_source))
    }
}

pub type ShaderSetResult = Option<()>;
const fn success() -> ShaderSetResult {
    Some(())
}

const fn failure() -> ShaderSetResult {
    None
}

/// Header: https://learnopengl.com/code_viewer_gh.php?code=src/7.in_practice/3.2d_game/0.full_source/shader.h
/// Source: https://learnopengl.com/code_viewer_gh.php?code=src/7.in_practice/3.2d_game/0.full_source/shader.cpp
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shader {
    id: u32,
}

impl Shader {
    const DEBUG: bool = true;

    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn is_init(&self) -> bool {
        self.id != 0
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn compile(&mut self, gl: &glitz::GlFns, args: &ShaderCompileArgs) -> bool {
        let (mut vertex_source, mut fragment_source, mut geometry_source) = match args.to_cstrings()
        {
            Some(tup) => tup,
            None => {
                eprintln!("Shader source Strings are not valid CStrings");
                return false;
            }
        };

        let mut vert_id: u32 = 0;
        let mut frag_id: u32 = 0;
        let mut geo_id: u32 = 0;

        // Compile vertex shader
        println!("creating vertex shader");
        vert_id = gl.CreateShader(glitz::GL_VERTEX_SHADER);
        unsafe {
            println!("setting vertex shader source");
            let mut vertex_bytes = vertex_source.into_bytes_with_nul();
            let single = vec![vertex_bytes.as_mut_ptr()];
            let vertex_ptr = single.as_ptr();
            gl.ShaderSource(vert_id, 1, vertex_ptr.cast(), std::ptr::null());
        }
        println!("compiling vertex shader");
        gl.CompileShader(vert_id);
        println!("checking vertex shader for errors");
        self.check_compile_errors(gl, vert_id, CompileType::Vertex, Self::DEBUG);

        // Compile fragment shader
        println!("creating frag shader");
        frag_id = gl.CreateShader(glitz::GL_FRAGMENT_SHADER);
        unsafe {
            println!("setting frag shader source");
            let mut frag_bytes = fragment_source.into_bytes_with_nul();
            let single = vec![frag_bytes.as_mut_ptr()];
            let frag_ptr = single.as_ptr();
            gl.ShaderSource(frag_id, 1, frag_ptr.cast(), std::ptr::null());
        }
        println!("compiling frag shader");
        gl.CompileShader(frag_id);
        println!("checking frag shader for errors");
        self.check_compile_errors(gl, frag_id, CompileType::Fragment, Self::DEBUG);

        // Compile geometry shader if provided
        if let Some(geo_str) = geometry_source {
            println!("creating geo shader");
            geo_id = gl.CreateShader(glitz::GL_GEOMETRY_SHADER);
            unsafe {
                println!("setting geo shader source");
                gl.ShaderSource(geo_id, 1, geo_str.as_ptr().cast(), std::ptr::null());
            }
            println!("compiling geo shader");
            gl.CompileShader(geo_id);
            println!("checking geo shader for errors");
            self.check_compile_errors(gl, geo_id, CompileType::Geometry, Self::DEBUG);
        } else {
            println!("no geo shader")
        }

        // Create shader program
        println!("creating shader program");
        self.id = gl.CreateProgram();
        println!("attaching vertex shader");
        gl.AttachShader(self.id, vert_id);
        println!("attaching frag shader");
        gl.AttachShader(self.id, frag_id);
        if args.has_geo() {
            println!("attaching geo shader");
            gl.AttachShader(self.id, geo_id);
        }
        println!("linking shader program");
        gl.LinkProgram(self.id);
        println!("checking program for errors");
        self.check_compile_errors(gl, self.id, CompileType::Program, Self::DEBUG);

        // Delete shaders now that they are linked
        println!("deleting linked shaders");
        gl.DeleteShader(vert_id);
        gl.DeleteShader(frag_id);
        if args.has_geo() {
            gl.DeleteShader(geo_id);
        }

        true
    }

    pub fn set_main(&self, gl: &glitz::GlFns) -> &Self {
        gl.UseProgram(self.id);
        self
    }

    pub fn set_float<S: AsRef<str>>(
        &self,
        gl: &glitz::GlFns,
        name: S,
        value: f32,
        use_shader: bool,
    ) -> ShaderSetResult {
        if use_shader {
            self.set_main(gl);
        }
        let name = name.as_ref();
        // All this just to make sure theres a stupid null terminator what bullshit
        let c_str = match CString::new(name.as_bytes()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Unable to create CString for {}: {}", name, e);
                return failure();
            }
        };
        let s_ptr = c_str.as_ptr();
        let location = unsafe { gl.GetUniformLocation(self.id, s_ptr) };
        if location < 1 {
            eprintln!("Unable to find location for {}", name);
            return failure();
        }
        gl.Uniform1f(location, value);
        success()
    }

    pub fn set_integer<S: AsRef<str>>(
        &self,
        gl: &glitz::GlFns,
        name: S,
        value: i32,
        use_shader: bool,
    ) -> ShaderSetResult {
        if use_shader {
            self.set_main(gl);
        }
        let name = name.as_ref();
        // All this just to make sure theres a stupid null terminator what bullshit
        let c_str = match CString::new(name.as_bytes()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Unable to create CString for {}: {}", name, e);
                return failure();
            }
        };
        let s_ptr = c_str.as_ptr();
        let location = unsafe { gl.GetUniformLocation(self.id, s_ptr) };
        if location < 0 {
            eprintln!("Unable to find location for {}", name);
            return failure();
        }
        gl.Uniform1i(location, value);
        success()
    }

    pub fn set_vector2f<S: AsRef<str>>(
        &self,
        gl: &glitz::GlFns,
        name: S,
        v2: impl Into<Vec2F>,
        use_shader: bool,
    ) -> ShaderSetResult {
        if use_shader {
            self.set_main(gl);
        }
        let name = name.as_ref();
        // All this just to make sure theres a stupid null terminator what bullshit
        let c_str = match CString::new(name.as_bytes()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Unable to create CString for {}: {}", name, e);
                return failure();
            }
        };
        let s_ptr = c_str.as_ptr();
        let location = unsafe { gl.GetUniformLocation(self.id, s_ptr) };
        if location < 0 {
            eprintln!("Unable to find location for {}", name);
            return failure();
        }
        let v2 = v2.into();
        gl.Uniform2f(location, v2.x, v2.y);
        success()
    }

    pub fn set_vector3f<S: AsRef<str>>(
        &self,
        gl: &glitz::GlFns,
        name: S,
        v3: impl Into<Vec3F>,
        use_shader: bool,
    ) -> ShaderSetResult {
        if use_shader {
            self.set_main(gl);
        }
        let name = name.as_ref();
        // All this just to make sure theres a stupid null terminator what bullshit
        let c_str = match CString::new(name.as_bytes()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Unable to create CString for {}: {}", name, e);
                return failure();
            }
        };
        let s_ptr = c_str.as_ptr();
        let location = unsafe { gl.GetUniformLocation(self.id, s_ptr) };
        if location < 0 {
            eprintln!("Unable to find location for {}", name);
            return failure();
        }
        let v3 = v3.into();
        gl.Uniform3f(location, v3.x, v3.y, v3.z);
        success()
    }

    pub fn set_vector4f<S: AsRef<str>>(
        &self,
        gl: &glitz::GlFns,
        name: S,
        v4: impl Into<Vec4F>,
        use_shader: bool,
    ) -> ShaderSetResult {
        if use_shader {
            self.set_main(gl);
        }
        let name = name.as_ref();
        // All this just to make sure theres a stupid null terminator what bullshit
        let c_str = match CString::new(name.as_bytes()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Unable to create CString for {}: {}", name, e);
                return failure();
            }
        };
        let s_ptr = c_str.as_ptr();
        let location = unsafe { gl.GetUniformLocation(self.id, s_ptr) };
        if location < 0 {
            eprintln!("Unable to find location for {}", name);
            return failure();
        }
        let v4 = v4.into();
        gl.Uniform4f(location, v4.x, v4.y, v4.z, v4.w);
        success()
    }

    pub fn set_matrix4f<S: AsRef<str>>(
        &self,
        gl: &glitz::GlFns,
        name: S,
        m: impl Into<[f32; 16]>,
        use_shader: bool,
    ) -> ShaderSetResult {
        if use_shader {
            self.set_main(gl);
        }
        let name = name.as_ref();
        // All this just to make sure theres a stupid null terminator what bullshit
        let c_str = match CString::new(name.as_bytes()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Unable to create CString for {}: {}", name, e);
                return failure();
            }
        };
        let s_ptr = c_str.as_ptr();
        let location = unsafe { gl.GetUniformLocation(self.id, s_ptr) };
        if location < 0 {
            eprintln!("Unable to find location for {}", name);
            return failure();
        }
        let flat: [f32; 16] = m.into();
        let ptr = flat.as_ptr().cast();
        unsafe {
            gl.UniformMatrix4fv(location, 1, 0, ptr);
        }
        success()
    }

    pub fn set_matrix4f_from<S: AsRef<str>>(
        &self,
        gl: &glitz::GlFns,
        name: S,
        m: impl Into<Mat4F>,
        use_shader: bool,
    ) -> ShaderSetResult {
        if use_shader {
            self.set_main(gl);
        }
        let name = name.as_ref();
        // All this just to make sure theres a stupid null terminator what bullshit
        let c_str = match CString::new(name.as_bytes()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Unable to create CString for {}: {}", name, e);
                return failure();
            }
        };
        let s_ptr = c_str.as_ptr();
        let location = unsafe { gl.GetUniformLocation(self.id, s_ptr) };
        if location < 0 {
            eprintln!("Unable to find location for {}", name);
            return failure();
        }
        let mat: Mat4F = m.into();
        let ptr = mat.as_ptr().cast();
        unsafe {
            gl.UniformMatrix4fv(location, 1, 0, ptr);
        }
        success()
    }

    pub fn set_matrix4f_from_ptr<S: AsRef<str>>(
        &self,
        gl: &glitz::GlFns,
        name: S,
        m: *const [f32; 16],
        use_shader: bool,
    ) -> ShaderSetResult {
        if use_shader {
            self.set_main(gl);
        }
        let name = name.as_ref();
        // All this just to make sure theres a stupid null terminator what bullshit
        let c_str = match CString::new(name.as_bytes()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Unable to create CString for {}: {}", name, e);
                return failure();
            }
        };
        let s_ptr = c_str.as_ptr();
        let location = unsafe { gl.GetUniformLocation(self.id, s_ptr) };
        if location < 0 {
            eprintln!("Unable to find location for {}", name);
            return failure();
        }
        unsafe {
            gl.UniformMatrix4fv(location, 1, 0, m);
        }
        success()
    }

    fn check_compile_errors(&self, gl: &glitz::GlFns, id: u32, check: CompileType, debug: bool) {
        let mut success = 0;
        if check.is_program() {
            unsafe {
                gl.GetProgramiv(id, glitz::GL_LINK_STATUS, &mut success);
            }
        } else {
            unsafe {
                gl.GetShaderiv(id, glitz::GL_COMPILE_STATUS, &mut success);
            }
        }

        if success == 0 {
            println!(
                "| ERROR: Shader {} (CompileType: {})",
                if check.is_program() {
                    "Link-time Error"
                } else {
                    "Compile-time Error"
                },
                check
            );
            println!("Attempting to get info log...");
            match if check.is_program() {
                super::util::get_program_info_log(gl, id, true)
            } else {
                super::util::get_shader_info_log(gl, id, true)
            } {
                Ok(log) => println!("Info Log: {}", log),
                Err(err) => println!("Error getting info log: {}", err),
            }

            println!("-- --------------------------------------------------- -- ");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    const LONG_STR_1: &str = "Rand is mature (suitable for general usage, with infrequent breaking releases which minimise breakage) but not yet at 1.0. We maintain compatibility with pinned versions of the Rust compiler (see below).

Current Rand versions are:

Version 0.7 was released in June 2019, moving most non-uniform distributions to an external crate, moving from_entropy to SeedableRng, and many small changes and fixes.
Version 0.8 was released in December 2020 with many small changes.
A detailed changelog is available for releases.

When upgrading to the next minor series (especially 0.4 → 0.5), we recommend reading the Upgrade Guide.

Rand has not yet reached 1.0 implying some breaking changes may arrive in the future (SemVer allows each 0.x.0 release to include breaking changes), but is considered mature: breaking changes are minimised and breaking releases are infrequent.

Rand libs have inter-dependencies and make use of the semver trick in order to make traits compatible across crate versions. (This is especially important for RngCore and SeedableRng.) A few crate releases are thus compatibility shims, depending on the next lib version (e.g. rand_core versions 0.2.2 and 0.3.1). This means, for example, that rand_core_0_4_0::SeedableRng and rand_core_0_3_0::SeedableRng are distinct, incompatible traits, which can cause build errors. Usually, running cargo update is enough to fix any issues.";

    const LONG_STR_1_SIZE: usize = LONG_STR_1.len();

    const LONG_STR_2: &str = "Rand is built with these features enabled by default:

std enables functionality dependent on the std lib
alloc (implied by std) enables functionality requiring an allocator
getrandom (implied by std) is an optional dependency providing the code behind rngs::OsRng
std_rng enables inclusion of StdRng, thread_rng and random (the latter two also require that std be enabled)
Optionally, the following dependencies can be enabled:

log enables logging via the log crate
Additionally, these features configure Rand:

small_rng enables inclusion of the SmallRng PRNG
nightly enables some optimizations requiring nightly Rust
simd_support (experimental) enables sampling of SIMD values (uniformly random SIMD integers and floats), requiring nightly Rust
min_const_gen enables generating random arrays of any size using min-const-generics, requiring Rust ≥ 1.51.
Note that nightly features are not stable and therefore not all library and compiler versions will be compatible. This is especially true of Rand's experimental simd_support feature.

Rand supports limited functionality in no_std mode (enabled via default-features = false). In this case, OsRng and from_entropy are unavailable (unless getrandom is enabled), large parts of seq are unavailable (unless alloc is enabled), and thread_rng and random are unavailable.";
    const LONG_STR_2_SIZE: usize = LONG_STR_2.len();

    const LONG_STR_3: &str = "                              Apache License
                        Version 2.0, January 2004
                     https://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   \"License\" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   \"Licensor\" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   \"Legal Entity\" shall mean the union of the acting entity and all
   other entities that control, are controlled by, or are under common
   control with that entity. For the purposes of this definition,
   \"control\" means (i) the power, direct or indirect, to cause the
   direction or management of such entity, whether by contract or
   otherwise, or (ii) ownership of fifty percent (50%) or more of the
   outstanding shares, or (iii) beneficial ownership of such entity.

   \"You\" (or \"Your\") shall mean an individual or Legal Entity
   exercising permissions granted by this License.

   \"Source\" form shall mean the preferred form for making modifications,
   including but not limited to software source code, documentation
   source, and configuration files.

   \"Object\" form shall mean any form resulting from mechanical
   transformation or translation of a Source form, including but
   not limited to compiled object code, generated documentation,
   and conversions to other media types.

   \"Work\" shall mean the work of authorship, whether in Source or
   Object form, made available under the License, as indicated by a
   copyright notice that is included in or attached to the work
   (an example is provided in the Appendix below).

   \"Derivative Works\" shall mean any work, whether in Source or Object
   form, that is based on (or derived from) the Work and for which the
   editorial revisions, annotations, elaborations, or other modifications
   represent, as a whole, an original work of authorship. For the purposes
   of this License, Derivative Works shall not include works that remain
   separable from, or merely link (or bind by name) to the interfaces of,
   the Work and Derivative Works thereof.

   \"Contribution\" shall mean any work of authorship, including
   the original version of the Work and any modifications or additions
   to that Work or Derivative Works thereof, that is intentionally
   submitted to Licensor for inclusion in the Work by the copyright owner
   or by an individual or Legal Entity authorized to submit on behalf of
   the copyright owner. For the purposes of this definition, \"submitted\"
   means any form of electronic, verbal, or written communication sent
   to the Licensor or its representatives, including but not limited to
   communication on electronic mailing lists, source code control systems,
   and issue tracking systems that are managed by, or on behalf of, the
   Licensor for the purpose of discussing and improving the Work, but
   excluding communication that is conspicuously marked or otherwise
   designated in writing by the copyright owner as \"Not a Contribution.\"

   \"Contributor\" shall mean Licensor and any individual or Legal Entity
   on behalf of whom a Contribution has been received by Licensor and
   subsequently incorporated within the Work.

2. Grant of Copyright License. Subject to the terms and conditions of
   this License, each Contributor hereby grants to You a perpetual,
   worldwide, non-exclusive, no-charge, royalty-free, irrevocable
   copyright license to reproduce, prepare Derivative Works of,
   publicly display, publicly perform, sublicense, and distribute the
   Work and such Derivative Works in Source or Object form.

3. Grant of Patent License. Subject to the terms and conditions of
   this License, each Contributor hereby grants to You a perpetual,
   worldwide, non-exclusive, no-charge, royalty-free, irrevocable
   (except as stated in this section) patent license to make, have made,
   use, offer to sell, sell, import, and otherwise transfer the Work,
   where such license applies only to those patent claims licensable
   by such Contributor that are necessarily infringed by their
   Contribution(s) alone or by combination of their Contribution(s)
   with the Work to which such Contribution(s) was submitted. If You
   institute patent litigation against any entity (including a
   cross-claim or counterclaim in a lawsuit) alleging that the Work
   or a Contribution incorporated within the Work constitutes direct
   or contributory patent infringement, then any patent licenses
   granted to You under this License for that Work shall terminate
   as of the date such litigation is filed.

4. Redistribution. You may reproduce and distribute copies of the
   Work or Derivative Works thereof in any medium, with or without
   modifications, and in Source or Object form, provided that You
   meet the following conditions:

   (a) You must give any other recipients of the Work or
       Derivative Works a copy of this License; and

   (b) You must cause any modified files to carry prominent notices
       stating that You changed the files; and

   (c) You must retain, in the Source form of any Derivative Works
       that You distribute, all copyright, patent, trademark, and
       attribution notices from the Source form of the Work,
       excluding those notices that do not pertain to any part of
       the Derivative Works; and

   (d) If the Work includes a \"NOTICE\" text file as part of its
       distribution, then any Derivative Works that You distribute must
       include a readable copy of the attribution notices contained
       within such NOTICE file, excluding those notices that do not
       pertain to any part of the Derivative Works, in at least one
       of the following places: within a NOTICE text file distributed
       as part of the Derivative Works; within the Source form or
       documentation, if provided along with the Derivative Works; or,
       within a display generated by the Derivative Works, if and
       wherever such third-party notices normally appear. The contents
       of the NOTICE file are for informational purposes only and
       do not modify the License. You may add Your own attribution
       notices within Derivative Works that You distribute, alongside
       or as an addendum to the NOTICE text from the Work, provided
       that such additional attribution notices cannot be construed
       as modifying the License.

   You may add Your own copyright statement to Your modifications and
   may provide additional or different license terms and conditions
   for use, reproduction, or distribution of Your modifications, or
   for any such Derivative Works as a whole, provided Your use,
   reproduction, and distribution of the Work otherwise complies with
   the conditions stated in this License.

5. Submission of Contributions. Unless You explicitly state otherwise,
   any Contribution intentionally submitted for inclusion in the Work
   by You to the Licensor shall be under the terms and conditions of
   this License, without any additional terms or conditions.
   Notwithstanding the above, nothing herein shall supersede or modify
   the terms of any separate license agreement you may have executed
   with Licensor regarding such Contributions.

6. Trademarks. This License does not grant permission to use the trade
   names, trademarks, service marks, or product names of the Licensor,
   except as required for reasonable and customary use in describing the
   origin of the Work and reproducing the content of the NOTICE file.

7. Disclaimer of Warranty. Unless required by applicable law or
   agreed to in writing, Licensor provides the Work (and each
   Contributor provides its Contributions) on an \"AS IS\" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
   implied, including, without limitation, any warranties or conditions
   of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
   PARTICULAR PURPOSE. You are solely responsible for determining the
   appropriateness of using or redistributing the Work and assume any
   risks associated with Your exercise of permissions under this License.

8. Limitation of Liability. In no event and under no legal theory,
   whether in tort (including negligence), contract, or otherwise,
   unless required by applicable law (such as deliberate and grossly
   negligent acts) or agreed to in writing, shall any Contributor be
   liable to You for damages, including any direct, indirect, special,
   incidental, or consequential damages of any character arising as a
   result of this License or out of the use or inability to use the
   Work (including but not limited to damages for loss of goodwill,
   work stoppage, computer failure or malfunction, or any and all
   other commercial damages or losses), even if such Contributor
   has been advised of the possibility of such damages.

9. Accepting Warranty or Additional Liability. While redistributing
   the Work or Derivative Works thereof, You may choose to offer,
   and charge a fee for, acceptance of support, warranty, indemnity,
   or other liability obligations and/or rights consistent with this
   License. However, in accepting such obligations, You may act only
   on Your own behalf and on Your sole responsibility, not on behalf
   of any other Contributor, and only if You agree to indemnify,
   defend, and hold each Contributor harmless for any liability
   incurred by, or claims asserted against, such Contributor by reason
   of your accepting any such warranty or additional liability.

END OF TERMS AND CONDITIONS";
    const LONG_STR_3_SIZE: usize = LONG_STR_3.len();

    #[test]
    fn bytes_vs_chars() {
        use std::time::{Duration, Instant};

        println!("Testing bytes vs chars for string validation...\n");

        // Long string 1
        let mut is_valid = true;
        let start = Instant::now();
        for bytes in LONG_STR_1.bytes() {
            if bytes == 0 {
                is_valid = false;
                break;
            }
        }
        let byte_duration1 = start.elapsed();
        let byte_valid1 = is_valid;

        is_valid = true;
        let start = Instant::now();
        for chars in LONG_STR_1.chars() {
            if chars == '\0' {
                is_valid = false;
                break;
            }
        }
        let char_duration1 = start.elapsed();
        let char_valid1 = is_valid;

        // Long string 2
        is_valid = true;
        let start = Instant::now();
        for bytes in LONG_STR_2.bytes() {
            if bytes == 0 {
                is_valid = false;
                break;
            }
        }
        let byte_duration2 = start.elapsed();
        let byte_valid2 = is_valid;

        is_valid = true;
        let start = Instant::now();
        for chars in LONG_STR_2.chars() {
            if chars == '\0' {
                is_valid = false;
                break;
            }
        }
        let char_duration2 = start.elapsed();
        let char_valid2 = is_valid;

        // Long string 3
        is_valid = true;
        let start = Instant::now();
        for bytes in LONG_STR_3.bytes() {
            if bytes == 0 {
                is_valid = false;
                break;
            }
        }
        let byte_duration3 = start.elapsed();
        let byte_valid3 = is_valid;

        is_valid = true;
        let start = Instant::now();
        for chars in LONG_STR_3.chars() {
            if chars == '\0' {
                is_valid = false;
                break;
            }
        }
        let char_duration3 = start.elapsed();
        let char_valid3 = is_valid;

        println!("TEST 1: String Length = {}", LONG_STR_1_SIZE);
        println!(
            "\tByte iteration produced a result that {} valid in {:>10?}",
            if byte_valid1 { "WAS" } else { "WAS NOT" },
            byte_duration1
        );
        println!(
            "\tChar iteration produced a result that {} valid in {:>10?}",
            if char_valid1 { "WAS" } else { "WAS NOT" },
            char_duration1
        );
        println!();
        println!("TEST 2: String Length = {}", LONG_STR_2_SIZE);
        println!(
            "\tByte iteration produced a result that {} valid in {:>10?}",
            if byte_valid2 { "WAS" } else { "WAS NOT" },
            byte_duration2
        );
        println!(
            "\tChar iteration produced a result that {} valid in {:>10?}",
            if char_valid2 { "WAS" } else { "WAS NOT" },
            char_duration2
        );
        println!();
        println!("TEST 3: String Length = {}", LONG_STR_3_SIZE);
        println!(
            "\tByte iteration produced a result that {} valid in {:>10?}",
            if byte_valid3 { "WAS" } else { "WAS NOT" },
            byte_duration3
        );
        println!(
            "\tChar iteration produced a result that {} valid in {:>10?}",
            if char_valid3 { "WAS" } else { "WAS NOT" },
            char_duration3
        );

        let cs1 = CString::new(LONG_STR_1.as_bytes());
        assert!(cs1.is_ok());
        assert_eq!(cs1.unwrap().as_bytes().len(), LONG_STR_1_SIZE);
        let cs2 = CString::new(LONG_STR_2.as_bytes());
        assert!(cs2.is_ok());
        assert_eq!(cs2.unwrap().as_bytes().len(), LONG_STR_2_SIZE);
        let cs3 = CString::new(LONG_STR_3.as_bytes());
        assert!(cs3.is_ok());
        assert_eq!(cs3.unwrap().as_bytes().len(), LONG_STR_3_SIZE);
    }
}
