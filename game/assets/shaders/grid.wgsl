#import bevy_sprite::mesh2d_view_bindings::globals 
#import bevy_render::view::View
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(0) @binding(0) var<uniform> view: View;

fn box(uv: vec2<f32>, _size: f32) -> f32 {
    var size = vec2(0.5) - _size * 0.5;
    var box = smoothstep(size, size + vec2(0.01), uv);
    box *= smoothstep(size, size + vec2(0.01), (1.0 - uv));

    return box.x * box.y;
}

fn tile(uv: vec2<f32>, zoom: f32) -> vec2<f32> {
    return fract(uv * zoom);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Put (0,0) at the centre of the screen
    var uv = (mesh.uv.xy * 2.0) - 1.0;

    uv = tile(uv, 400.0);

    let color = vec3<f32>(box(uv, 0.98));

	return vec4f(color, 1.0);
}    
    
