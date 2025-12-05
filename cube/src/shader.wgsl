// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.color = model.color;
    out.normal = model.normal;
    out.world_position = model.position;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Light direction (simulating sun from top-right)
    let light_dir = normalize(vec3<f32>(1.0, 1.5, 1.0));
    
    // Ambient lighting
    let ambient_strength = 0.3;
    let ambient = ambient_strength * vec3<f32>(1.0, 1.0, 1.0);
    
    // Diffuse lighting
    let norm = normalize(in.normal);
    let diff = max(dot(norm, light_dir), 0.0);
    let diffuse = diff * vec3<f32>(1.0, 1.0, 1.0);
    
    // Simple shadow approximation based on surface orientation
    // Surfaces facing down get darker (simulating shadow)
    let shadow_factor = smoothstep(-0.5, 0.5, in.normal.y);
    let shadow = mix(0.4, 1.0, shadow_factor);
    
    // Combine lighting
    let lighting = (ambient + diffuse) * shadow;
    let result = in.color.rgb * lighting;
    
    return vec4<f32>(result, in.color.a);
}

