#import bevy_sprite::mesh2d_view_bindings::globals 
#import bevy_render::view::View
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1) var<uniform> base_color: vec4<f32>;
@group(2) @binding(2) var<storage> colliders: array<vec4<f32>>;

const SPEED:f32 = 1.0;
const PI: f32 = 3.1415;

fn ncos(in: f32) -> f32 {
    return (cos(in) + 1.0) / 2.0;
}

fn nsin(in: f32) -> f32 {
    return (sin(in) + 1.0) / 2.0;
}

fn smooth_union(distance_a: f32, distance_b: f32, k: f32) -> f32 {
    let h = clamp(0.5 - 0.5 * (distance_b - distance_a) / k, 0., 1.);
    return mix(distance_b, distance_a, h) - k * h * (1. - h);
}

fn sd_circle(uv: vec2<f32>, center: vec2<f32>, radius: f32) -> f32 {
    return 1.0 - sign(distance(uv, center) - radius);
}

fn circle(position: vec2<f32>, radius: f32) -> f32 {
    return length(position) - radius;
}

fn intersect(shape1: f32, shape2: f32) -> f32 {
    return max(shape1, shape2);
}

fn round_intersect(shape_a: f32, shape_b: f32, radius: f32) -> f32 {
    let intersection_space = vec2(max(shape_a + radius, 0.0), max(shape_b + radius, 0.0));
    let outside_distance = length(intersection_space);
    let simple_intersection = intersect(shape_a, shape_b);
    let inside_distance = min(simple_intersection, -radius);
    return outside_distance + inside_distance;
}

fn subtract(base: f32, subtraction: f32) -> f32 {
    return intersect(base, -subtraction);
}

fn round_subtract(base: f32, subtraction: f32, radius: f32) -> f32 {
    return round_intersect(base, -subtraction, radius);
}

fn translate(sample_position: vec2<f32>, offset: vec2<f32>) -> vec2<f32> {
    return sample_position - offset;
}


fn merge(shape_a: f32, shape_b: f32) -> f32 {
    return min(shape_a, shape_b);
}

fn sum_of_sines(normalized_angle: f32, time: f32, magnitude: f32) -> f32 {
    var sin_extra = nsin(normalized_angle * 80.0 * PI + 30.0 * time);
    sin_extra += nsin(normalized_angle * 2.0 * PI + time);
    sin_extra /= 2.0;
    sin_extra *= magnitude;
    return sin_extra;
}

fn scene(sample_position: vec2<f32>, time: f32) -> f32 {
    let circle_position_a = translate(sample_position, vec2(0.0, 0.0));
    let angle_a = atan2(circle_position_a.x, circle_position_a.y);
    let normalized_angle_a = (angle_a + PI) / (2.0 * PI);
    // let sin_extra_a = sum_of_sines(normalized_angle_a, time, 0.01);
    let circle_a = circle(circle_position_a, 1.0);
    var scene_distance = circle_a;
    let array_length = i32(arrayLength(&colliders));

    for (var i = 1; i < array_length; i++) {
        let current_collider = colliders[i];
        let circle_position_b = translate(sample_position, current_collider.xy);
        let angle_b = atan2(circle_position_b.x, circle_position_b.y);
        let normalized_angle_b = (angle_b + PI) / (2.0 * PI);
        // let sin_extra_b = 0.05 * nsin(normalized_angle_b * 80.0 * PI + 30.0 * time);
        let sin_extra_b = 0.0;
        let circle_b = circle(circle_position_b, current_collider.z + 0.0 + sin_extra_b);
        scene_distance = max(scene_distance, round_subtract(circle_a, circle_b, 0.2));
    }

    return scene_distance;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let inside_color = base_color;
    let border_color = vec4(inside_color.rgb * 0.5, inside_color.a);
    let outside_color = vec4(0.0, 0.5, 0.0, 0.0);

    let line_distance = 0.05;
    let line_thickness = 0.05;

    var uv = (mesh.uv.xy * 2.0) - 1.0;
    let t = globals.time * SPEED;

    let scene_distance = scene(uv, t);
    var color = mix(border_color, outside_color, step(0.0, scene_distance));
    color = mix(inside_color, color, step(0.0, scene_distance + 0.1));

    let scene_distance_change = fwidth(scene_distance) * 0.5;
    // let major_line_distance = abs(fract(scene_distance / line_distance + 0.5) -0.5) * line_distance;
    let major_line_distance = abs(fract(scene_distance / line_distance + 0.5) - 0.5);
    let major_lines = smoothstep(line_thickness - scene_distance_change, line_thickness + scene_distance_change, major_line_distance);

    // color *= major_lines;

	return color;
}
