// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub struct FlatMatrix4<T>([T; 16]);

impl<T> FlatMatrix4<T> {
    pub fn new(m: [T; 16]) -> Self {
        Self(m)
    }

    pub fn data(&self) -> &[T; 16] {
        &self.0
    }

    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.0.as_mut_slice()
    }

    pub fn as_array(&self) -> &[T; 16] {
        &self.0
    }

    pub fn as_mut_array(&mut self) -> &mut [T; 16] {
        &mut self.0
    }

    pub fn into_matrix4(self) -> Matrix4<T> {
        let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15] = self.0;
        let x = Vector4::new(v0, v1, v2, v3);
        let y = Vector4::new(v4, v5, v6, v7);
        let z = Vector4::new(v8, v9, v10, v11);
        let w = Vector4::new(v12, v13, v14, v15);
        Matrix4::from_cols(x, y, z, w)
    }
}

impl<T: Clone> Clone for FlatMatrix4<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Copy> Copy for FlatMatrix4<T> {}

impl<T: Copy> std::ops::Index<usize> for FlatMatrix4<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        if index < 4 {
            &self.0[index * 4..(index + 1) * 4]
        } else {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                4, index
            );
        }
    }
}

impl<T: Clone> FlatMatrix4<T> {
    pub fn from_cols_cloned(m: &[[&T; 4]; 4]) -> Self {
        Self([
            m[0][0].clone(),
            m[1][0].clone(),
            m[2][0].clone(),
            m[3][0].clone(),
            m[0][1].clone(),
            m[1][1].clone(),
            m[2][1].clone(),
            m[3][1].clone(),
            m[0][2].clone(),
            m[1][2].clone(),
            m[2][2].clone(),
            m[3][2].clone(),
            m[0][3].clone(),
            m[1][3].clone(),
            m[2][3].clone(),
            m[3][3].clone(),
        ])
    }

    pub fn clone_to_matrix4(&self) -> Matrix4<T> {
        let clone = self.clone();
        clone.into_matrix4()
    }
}

impl<T: Copy> FlatMatrix4<T> {
    pub fn from_cols(m: [[T; 4]; 4]) -> Self {
        let [x, y, z, w] = m;
        Self([
            x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3], z[0], z[1], z[2], z[3], w[0], w[1],
            w[2], w[3],
        ])
    }

    fn from_cols_ref(m: &[[T; 4]; 4]) -> Self {
        Self([
            m[0][0], m[1][0], m[2][0], m[3][0], m[0][1], m[1][1], m[2][1], m[3][1], m[0][2],
            m[1][2], m[2][2], m[3][2], m[0][3], m[1][3], m[2][3], m[3][3],
        ])
    }

    fn from_cols_ref_array(m: [[&T; 4]; 4]) -> Self {
        Self([
            *m[0][0], *m[1][0], *m[2][0], *m[3][0], *m[0][1], *m[1][1], *m[2][1], *m[3][1],
            *m[0][2], *m[1][2], *m[2][2], *m[3][2], *m[0][3], *m[1][3], *m[2][3], *m[3][3],
        ])
    }

    fn from_cols_ref_array_ref(m: &[[&T; 4]; 4]) -> Self {
        Self([
            *m[0][0], *m[1][0], *m[2][0], *m[3][0], *m[0][1], *m[1][1], *m[2][1], *m[3][1],
            *m[0][2], *m[1][2], *m[2][2], *m[3][2], *m[0][3], *m[1][3], *m[2][3], *m[3][3],
        ])
    }
}

// CGMath Matrix4 Type
impl<T: Copy> From<Matrix4<T>> for FlatMatrix4<T> {
    fn from(matrix: Matrix4<T>) -> Self {
        Self([
            matrix.x[0],
            matrix.x[1],
            matrix.x[2],
            matrix.x[3],
            matrix.y[0],
            matrix.y[1],
            matrix.y[2],
            matrix.y[3],
            matrix.z[0],
            matrix.z[1],
            matrix.z[2],
            matrix.z[3],
            matrix.w[0],
            matrix.w[1],
            matrix.w[2],
            matrix.w[3],
        ])
    }
}
impl<T: Copy> From<&Matrix4<T>> for FlatMatrix4<T> {
    fn from(matrix: &Matrix4<T>) -> Self {
        Self([
            matrix.x[0],
            matrix.x[1],
            matrix.x[2],
            matrix.x[3],
            matrix.y[0],
            matrix.y[1],
            matrix.y[2],
            matrix.y[3],
            matrix.z[0],
            matrix.z[1],
            matrix.z[2],
            matrix.z[3],
            matrix.w[0],
            matrix.w[1],
            matrix.w[2],
            matrix.w[3],
        ])
    }
}
// Value Arrays
impl<T: Copy> From<[T; 16]> for FlatMatrix4<T> {
    fn from(m: [T; 16]) -> Self {
        Self(m)
    }
}
impl<T: Copy> From<&[T; 16]> for FlatMatrix4<T> {
    fn from(m: &[T; 16]) -> Self {
        Self(*m)
    }
}
// Column Arrays
impl<T: Copy> From<[[T; 4]; 4]> for FlatMatrix4<T> {
    fn from(m: [[T; 4]; 4]) -> Self {
        Self::from_cols(m)
    }
}
impl<T: Copy> From<[[&T; 4]; 4]> for FlatMatrix4<T> {
    fn from(m: [[&T; 4]; 4]) -> Self {
        Self::from_cols_ref_array(m)
    }
}
impl<T: Copy> From<&[[T; 4]; 4]> for FlatMatrix4<T> {
    fn from(m: &[[T; 4]; 4]) -> Self {
        Self::from_cols_ref(m)
    }
}
impl<T: Copy> From<&[[&T; 4]; 4]> for FlatMatrix4<T> {
    fn from(m: &[[&T; 4]; 4]) -> Self {
        Self::from_cols_ref_array_ref(m)
    }
}
// Value Slices
impl<T: Copy> TryFrom<&[T]> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(value: &[T]) -> Result<Self, Self::Error> {
        if value.len() != 16 {
            return Err(());
        }

        let data = [
            value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
            value[8], value[9], value[10], value[11], value[12], value[13], value[14], value[15],
        ];

        Ok(Self(data))
    }
}
impl<T: Copy> TryFrom<&[&T]> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(value: &[&T]) -> Result<Self, Self::Error> {
        if value.len() != 16 {
            return Err(());
        }

        let data = [
            *value[0], *value[1], *value[2], *value[3], *value[4], *value[5], *value[6], *value[7],
            *value[8], *value[9], *value[10], *value[11], *value[12], *value[13], *value[14],
            *value[15],
        ];

        Ok(Self(data))
    }
}
// Column Slices
impl<T: Copy> TryFrom<&[&[&T]]> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(values: &[&[&T]]) -> Result<Self, Self::Error> {
        if values.len() != 4 {
            return Err(());
        }
        for &value in values {
            if value.len() != 4 {
                return Err(());
            }
        }
        let x = values[0];
        let y = values[1];
        let z = values[2];
        let w = values[3];
        Ok(Self([
            *x[0], *x[1], *x[2], *x[3], *y[0], *y[1], *y[2], *y[3], *z[0], *z[1], *z[2], *z[3],
            *w[0], *w[1], *w[2], *w[3],
        ]))
    }
}
impl<T: Copy> TryFrom<&[&[T]]> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(values: &[&[T]]) -> Result<Self, Self::Error> {
        if values.len() != 4 {
            return Err(());
        }
        for &value in values {
            if value.len() != 4 {
                return Err(());
            }
        }
        let x = values[0];
        let y = values[1];
        let z = values[2];
        let w = values[3];
        Ok(Self([
            x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3], z[0], z[1], z[2], z[3], w[0], w[1],
            w[2], w[3],
        ]))
    }
}
// Value Vecs
impl<T: Copy> TryFrom<Vec<T>> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() != 16 {
            return Err(());
        }

        let data = [
            value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
            value[8], value[9], value[10], value[11], value[12], value[13], value[14], value[15],
        ];

        Ok(Self(data))
    }
}
impl<T: Copy> TryFrom<&Vec<T>> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(value: &Vec<T>) -> Result<Self, Self::Error> {
        if value.len() != 16 {
            return Err(());
        }

        let data = [
            value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
            value[8], value[9], value[10], value[11], value[12], value[13], value[14], value[15],
        ];

        Ok(Self(data))
    }
}
impl<T: Copy> TryFrom<Vec<&T>> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(value: Vec<&T>) -> Result<Self, Self::Error> {
        if value.len() != 16 {
            return Err(());
        }

        let data = [
            *value[0], *value[1], *value[2], *value[3], *value[4], *value[5], *value[6], *value[7],
            *value[8], *value[9], *value[10], *value[11], *value[12], *value[13], *value[14],
            *value[15],
        ];

        Ok(Self(data))
    }
}
impl<T: Copy> TryFrom<&Vec<&T>> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(value: &Vec<&T>) -> Result<Self, Self::Error> {
        if value.len() != 16 {
            return Err(());
        }

        let data = [
            *value[0], *value[1], *value[2], *value[3], *value[4], *value[5], *value[6], *value[7],
            *value[8], *value[9], *value[10], *value[11], *value[12], *value[13], *value[14],
            *value[15],
        ];

        Ok(Self(data))
    }
}
// Column Vecs - This is Hilarious BTW
impl<T: Copy> TryFrom<Vec<Vec<T>>> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(values: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        if values.len() != 4 {
            return Err(());
        }
        for value in &values {
            if value.len() != 4 {
                return Err(());
            }
        }
        let x = &values[0];
        let y = &values[1];
        let z = &values[2];
        let w = &values[3];
        Ok(Self([
            x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3], z[0], z[1], z[2], z[3], w[0], w[1],
            w[2], w[3],
        ]))
    }
}
impl<T: Copy> TryFrom<Vec<Vec<&T>>> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(values: Vec<Vec<&T>>) -> Result<Self, Self::Error> {
        if values.len() != 4 {
            return Err(());
        }
        for value in &values {
            if value.len() != 4 {
                return Err(());
            }
        }
        let x = &values[0];
        let y = &values[1];
        let z = &values[2];
        let w = &values[3];
        Ok(Self([
            *x[0], *x[1], *x[2], *x[3], *y[0], *y[1], *y[2], *y[3], *z[0], *z[1], *z[2], *z[3],
            *w[0], *w[1], *w[2], *w[3],
        ]))
    }
}
impl<T: Copy> TryFrom<Vec<&Vec<T>>> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(values: Vec<&Vec<T>>) -> Result<Self, Self::Error> {
        if values.len() != 4 {
            return Err(());
        }
        for value in &values {
            if value.len() != 4 {
                return Err(());
            }
        }
        let x = values[0];
        let y = values[1];
        let z = values[2];
        let w = values[3];
        Ok(Self([
            x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3], z[0], z[1], z[2], z[3], w[0], w[1],
            w[2], w[3],
        ]))
    }
}
impl<T: Copy> TryFrom<Vec<&Vec<&T>>> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(values: Vec<&Vec<&T>>) -> Result<Self, Self::Error> {
        if values.len() != 4 {
            return Err(());
        }
        for &value in &values {
            if value.len() != 4 {
                return Err(());
            }
        }
        let x = &values[0];
        let y = &values[1];
        let z = &values[2];
        let w = &values[3];
        Ok(Self([
            *x[0], *x[1], *x[2], *x[3], *y[0], *y[1], *y[2], *y[3], *z[0], *z[1], *z[2], *z[3],
            *w[0], *w[1], *w[2], *w[3],
        ]))
    }
}
impl<T: Copy> TryFrom<&Vec<&Vec<T>>> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(values: &Vec<&Vec<T>>) -> Result<Self, Self::Error> {
        if values.len() != 4 {
            return Err(());
        }
        for &value in values {
            if value.len() != 4 {
                return Err(());
            }
        }
        let x = values[0];
        let y = values[1];
        let z = values[2];
        let w = values[3];
        Ok(Self([
            x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3], z[0], z[1], z[2], z[3], w[0], w[1],
            w[2], w[3],
        ]))
    }
}
impl<T: Copy> TryFrom<&Vec<&Vec<&T>>> for FlatMatrix4<T> {
    type Error = ();

    fn try_from(values: &Vec<&Vec<&T>>) -> Result<Self, Self::Error> {
        if values.len() != 4 {
            return Err(());
        }
        for &value in values {
            if value.len() != 4 {
                return Err(());
            }
        }
        let x = values[0];
        let y = values[1];
        let z = values[2];
        let w = values[3];
        Ok(Self([
            *x[0], *x[1], *x[2], *x[3], *y[0], *y[1], *y[2], *y[3], *z[0], *z[1], *z[2], *z[3],
            *w[0], *w[1], *w[2], *w[3],
        ]))
    }
}
