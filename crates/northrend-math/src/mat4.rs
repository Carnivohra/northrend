use std::ops::Mul;

use crate::Vec3;

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub columns: [[f32; 4]; 4],
}

impl Mat4 {
    pub const ZERO: Self = Self::from_cols(
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
    );

    pub const IDENTITY: Self = Self::from_cols(
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    );

    #[inline]
    pub const fn from_cols(
        x_axis: [f32; 4],
        y_axis: [f32; 4],
        z_axis: [f32; 4],
        w_axis: [f32; 4],
    ) -> Self {
        Self {
            columns: [x_axis, y_axis, z_axis, w_axis],
        }
    }

    #[inline]
    pub const fn from_translation(translation: Vec3) -> Self {
        Self::from_cols(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [translation.x, translation.y, translation.z, 1.0],
        )
    }

    #[inline]
    pub const fn from_scale(scale: Vec3) -> Self {
        Self::from_cols(
            [scale.x, 0.0, 0.0, 0.0],
            [0.0, scale.y, 0.0, 0.0],
            [0.0, 0.0, scale.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    #[inline]
    pub fn look_at_rh(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = (target - eye).normalize();
        let right = forward.cross(up).normalize();
        let up = right.cross(forward);

        Self::from_cols(
            [right.x, up.x, -forward.x, 0.0],
            [right.y, up.y, -forward.y, 0.0],
            [right.z, up.z, -forward.z, 0.0],
            [-right.dot(eye), -up.dot(eye), forward.dot(eye), 1.0],
        )
    }

    #[inline]
    pub fn perspective_rh_reverse_z(
        vertical_field_of_view: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Self {
        debug_assert!(vertical_field_of_view > 0.0);
        debug_assert!(aspect_ratio > 0.0);
        debug_assert!(near > 0.0);
        debug_assert!(far > near);

        let focal_length = 1.0 / (vertical_field_of_view * 0.5).tan();
        let depth = near / (far - near);

        Self::from_cols(
            [focal_length / aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, focal_length, 0.0, 0.0],
            [0.0, 0.0, depth, -1.0],
            [0.0, 0.0, far * depth, 0.0],
        )
    }

    #[inline]
    pub fn transform_point3(self, point: Vec3) -> Vec3 {
        Vec3::new(
            self.columns[0][0] * point.x
                + self.columns[1][0] * point.y
                + self.columns[2][0] * point.z
                + self.columns[3][0],

            self.columns[0][1] * point.x
                + self.columns[1][1] * point.y
                + self.columns[2][1] * point.z
                + self.columns[3][1],

            self.columns[0][2] * point.x
                + self.columns[1][2] * point.y
                + self.columns[2][2] * point.z
                + self.columns[3][2],
        )
    }

    #[inline]
    pub fn transform_vector3(self, vector: Vec3) -> Vec3 {
        Vec3::new(
            self.columns[0][0] * vector.x
                + self.columns[1][0] * vector.y
                + self.columns[2][0] * vector.z,

            self.columns[0][1] * vector.x
                + self.columns[1][1] * vector.y
                + self.columns[2][1] * vector.z,

            self.columns[0][2] * vector.x
                + self.columns[1][2] * vector.y
                + self.columns[2][2] * vector.z,
        )
    }

    #[inline]
    pub fn mul_mat4(self, rhs: Self) -> Self {
        let mut columns = [[0.0; 4]; 4];

        for (column, output) in columns.iter_mut().enumerate() {
            for (row, value) in output.iter_mut().enumerate() {
                *value = self.columns[0][row] * rhs.columns[column][0]
                    + self.columns[1][row] * rhs.columns[column][1]
                    + self.columns[2][row] * rhs.columns[column][2]
                    + self.columns[3][row] * rhs.columns[column][3];
            }
        }

        Self { columns }
    }
}

impl Mul for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_mat4(rhs)
    }
}
