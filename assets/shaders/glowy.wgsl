#import bevy_pbr::mesh_view_bindings view
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_vertex_output as mesh_vertex_output
#import bevy_pbr::prepass_io as prepass_io

#import bevy_pbr::pbr_deferred_functions as pbr_deferred_functions
#import bevy_pbr::pbr_types as pbr_types
#import bevy_pbr::utils PI

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

fn refract(I: vec3<f32>, N: vec3<f32>, eta: f32) -> vec3<f32> {
    let k = max((1.0 - eta * eta * (1.0 - dot(N, I) * dot(N, I))), 0.0);
    return eta * I - (eta * dot(N, I) + sqrt(k)) * N;
}

const TAU: f32 = 6.28318530717958647692528676655900577;

fn dir_to_equirectangular(dir: vec3<f32>) -> vec2<f32> {
    let x = atan2(dir.z, dir.x) / TAU + 0.5; // 0-1
    let y = acos(dir.y) / PI; // 0-1
    return vec2<f32>(x, y);
}

@fragment
#ifdef DEFERRED_PREPASS
fn fragment(in: prepass_io::FragmentInput) -> prepass_io::FragmentOutput {
#else
fn fragment(in: mesh_vertex_output::MeshVertexOutput) -> @location(0) vec4<f32> {
#endif
    var N = normalize(in.world_normal);
    var V = normalize(view.world_position.xyz - in.world_position.xyz);
    let NdotV = max(dot(N, V), 0.0001);
    var fresnel = clamp(1.0 - NdotV, 0.0, 1.0);
    fresnel = pow(fresnel, 5.0) * 2.0;

    let glow = pow(NdotV, 10.0) * 50.0;
    var col = vec3(0.0, 0.0, 0.0);
    
    col = mix(col, vec3(0.5, 0.1, 0.0), glow);

    let bump_coords = dir_to_equirectangular(N * vec3(1.0,-0.5,1.0) - vec3(0.0,0.5,0.0));
    let bump = textureSample(texture, texture_sampler, bump_coords).r;

    var reflect_coords = dir_to_equirectangular(reflect(-V, N));
    let reflection = textureSample(texture, texture_sampler, reflect_coords).rgb;

    var refract_coords = dir_to_equirectangular(refract(-V, N + bump * 2.0, 1.0/1.52));
    let refraction = textureSample(texture, texture_sampler, refract_coords).rgb;

    col = (col * refraction) + reflection * (fresnel + 0.05);

#ifdef DEFERRED_PREPASS
    var pbr_input = pbr_types::pbr_input_new();
    pbr_input.frag_coord = in.position;
    pbr_input.material.base_color = vec4(col, 1.0);
    pbr_input.material.flags |= pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT;
    var out: prepass_io::FragmentOutput;
    out.deferred = pbr_deferred_functions::deferred_gbuffer_from_pbr_input(pbr_input);
    out.deferred_lighting_pass_id = 1u;
    return out;
#else
    return vec4(col, 1.0);
#endif
}
