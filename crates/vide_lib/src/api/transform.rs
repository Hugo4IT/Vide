#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[derive(Debug, Clone)]
pub struct Transform {
    translation: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
    scale: cgmath::Vector3<f32>,
    cached_matrix: Option<cgmath::Matrix4<f32>>,
}

impl Transform {
    pub fn new(translation: cgmath::Vector3<f32>, rotation: cgmath::Quaternion<f32>, scale: cgmath::Vector3<f32>) -> Self {
        Self {
            translation,
            rotation,
            scale,
            ..Default::default()
        }
    }

    #[inline]
    pub fn get_translation(&self) -> cgmath::Vector3<f32> {
        self.translation
    }

    #[inline]
    pub fn translate(&mut self, by: (f32, f32, f32)) {
        self.translation += cgmath::Vector3::new(by.0, by.1, by.2);
        self.cached_matrix = None;
    }

    #[inline]
    pub fn set_translation(&mut self, new_translation: (f32, f32, f32)) {
        self.translation = cgmath::Vector3::new(new_translation.0, new_translation.1, new_translation.2);
        self.cached_matrix = None;
    }

    #[inline]
    pub fn get_rotation(&self) -> cgmath::Quaternion<f32> {
        self.rotation
    }

    #[inline]
    pub fn rotate_euler(&mut self, by: (f32, f32, f32)) {
        self.rotation += cgmath::Quaternion::from(cgmath::Euler { x: cgmath::Deg(by.0), y: cgmath::Deg(by.1), z: cgmath::Deg(by.2) });
        self.cached_matrix = None;
    }

    #[inline]
    pub fn set_rotation_euler(&mut self, new_rotation: (f32, f32, f32)) {
        self.rotation = cgmath::Quaternion::from(cgmath::Euler { x: cgmath::Deg(new_rotation.0), y: cgmath::Deg(new_rotation.1), z: cgmath::Deg(new_rotation.2) });
        self.cached_matrix = None;
    }

    #[inline]
    pub fn get_scale(&self) -> cgmath::Vector3<f32> {
        self.scale
    }

    #[inline]
    pub fn scale(&mut self, by: (f32, f32, f32)) {
        self.scale = cgmath::Vector3::new(self.scale.x * by.0, self.scale.y * by.1, self.scale.z * by.2);
        self.cached_matrix = None;
    }

    #[inline]
    pub fn set_scale(&mut self, new_scale: (f32, f32, f32)) {
        self.scale = cgmath::Vector3::new(new_scale.0, new_scale.1, new_scale.2);
        self.cached_matrix = None;
    }

    pub fn rebuild_matrix(&mut self, parent_matrix: cgmath::Matrix4<f32>) {
        self.cached_matrix = Some(
            cgmath::Matrix4::from_translation(self.translation)
            * cgmath::Matrix4::from(self.rotation)
            * cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
            * parent_matrix);
    }

    pub fn matrix(&mut self, parent_matrix: cgmath::Matrix4<f32>) -> cgmath::Matrix4<f32> {
        if self.cached_matrix.is_none() {
            self.rebuild_matrix(parent_matrix);
        }

        self.cached_matrix.unwrap()
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: cgmath::Vector3::new(0.0, 0.0, 0.0),
            rotation: <cgmath::Quaternion<f32> as cgmath::Rotation3>::from_angle_z(cgmath::Rad(0.0)),
            scale: cgmath::Vector3::new(1.0, 1.0, 1.0),
            cached_matrix: None,
        }
    }
}