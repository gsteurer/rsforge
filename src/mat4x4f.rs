pub struct Mat4x4f {
    data: [f32; 16],
}

impl Mat4x4f {
    pub fn new() -> Self {
        let data: [f32; 16] = [0.0; 16];
        Mat4x4f { data: data }
    }
}
