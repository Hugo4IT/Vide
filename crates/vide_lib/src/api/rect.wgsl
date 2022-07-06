struct TransformUniform {
    transform_matrix: mat4x4<f32>,
};

@group(0)
@binding(0)
var<uniform> transform_uniform: TransformUniform;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
};

struct InstanceInput {
    @location(5) matrix_0: vec4<f32>,
    @location(6) matrix_1: vec4<f32>,
    @location(7) matrix_2: vec4<f32>,
    @location(8) matrix_3: vec4<f32>,
    @location(9) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let instance_matrix = mat4x4<f32>(
        instance.matrix_0,
        instance.matrix_1,
        instance.matrix_2,
        instance.matrix_3,
    );

    let transform_matrix = transform_uniform.transform_matrix;

    var out: VertexOutput;
    out.color = instance.color;
    out.clip_position = transform_matrix * instance_matrix * vec4<f32>(model.position, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}