use half::f16;
use serde::Serialize;

use serde_webgpu::mat::mat4x4;
use serde_webgpu::serialize_webgpu_buffer;
use serde_webgpu::vec::vec4;

#[test]
fn base() {
    #[derive(Serialize)]
    struct Uniform {
        a: f16,
        b: mat4x4<f32>,
    }

    let uniform = Uniform {
        a: f16::from_f32(123.456),
        b: [
            vec4([1.0, 2.0, 3.0, 4.0]),
            vec4([4.0, 5.0, 7.0, 8.0]),
            vec4([1.0, 2.0, 3.0, 4.0]),
            vec4([5.0, 6.0, 7.0, 8.0]),
        ],
    };

    let buffer = serialize_webgpu_buffer(&uniform).unwrap();
    println!("{:#?}", buffer);
}
