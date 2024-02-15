@vertex
fn vertex(@location(0) position: vec2f) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 0.0, 1.0);
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return vec4<f32>(0.25, 1.0, 0.25, 1.0);
}