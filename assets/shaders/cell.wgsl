#import bevy_sprite::mesh2d_view_bindings::globals 
#import bevy_render::view::View
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// @group(0) @binding(0) var<uniform> view: View;
@group(2) @binding(0) var<uniform> normalized_cell_overflow_radius: f32;
@group(2) @binding(1) var<uniform> base_color: vec4<f32>;

const SPEED:f32 = 1.0;
const PI: f32 = 3.1415;

fn ncos(in: f32) -> f32 {
    return (cos(in) + 1.0) / 2.0;
}

fn nsin(in: f32) -> f32 {
    return (sin(in) + 1.0) / 2.0;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let t = globals.time * SPEED;

    // Put (0,0) at the centre of the screen
    var uv = (mesh.uv.xy * 2.0) - 1.0;

    // Pulsing color
    let color_transform = 1.0 - (0.25 * nsin(t));
    let color = base_color.rgb * color_transform;

    // Differentiate between "base" radius, and "overflow" radius.
    // The base radius describes the radius of the inner circle
    // The overflow radius is a buffer radius for the extra sin waves surrounding the circle
    let overflow_radius = normalized_cell_overflow_radius;
    let base_radius = 1.0 - overflow_radius;

    let angle = atan2(uv.x, uv.y);
    let sin_extra = overflow_radius * ((nsin(angle * 4.0 + t)) * 0.5 + ncos(angle * 12.0 + t) * 0.5);
    let opacity = 1.0 - sign(distance(uv, vec2(0.)) - (base_radius + sin_extra));

	return vec4f(color, opacity);
}    
    

