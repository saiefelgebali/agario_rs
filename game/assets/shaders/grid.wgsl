#import bevy_sprite::mesh2d_view_bindings::globals 
#import bevy_render::view::View
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(0) @binding(0) var<uniform> view: View;
@group(2) @binding(0) var<uniform> offset: vec2<f32>;

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
    let resolution = view.viewport.zw;
    uv.x *= resolution.x / resolution.y;

    // TODO: Cleanup this logic + figure out correct formula to keep speed of grid aligned with player speed
    uv.x = uv.x + ((offset.x / 2000.0) * 3.0);
    uv.y = uv.y - ((offset.y / 2000.0) * 3.0);
    uv = tile(uv, 4.0 * (resolution.x + resolution.y)/2000.0);

    let squares = vec3<f32>(box(uv, 0.98));
    let borders = 1.0 - squares;

    let square_color = vec3(0.9, 0.9, 0.95);
    let border_color = vec3(0.3);

    var color = mix(vec3(0.0), square_color, squares);
    color = mix(color, border_color, borders);

	return vec4f(color, 1.0);
}    
    
