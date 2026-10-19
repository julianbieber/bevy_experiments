// @material Wall

import bevy_pbr_generator::pbr_maps::{Surface, default_surface};
import bevy_pbr_generator::noise;

struct Params {
    // @group Colour
    tint:  vec3<f32>, // @ui color srgb(0.4, 0.4, 0.45)
    // @group Surface
    scale: f32,       // @ui 8.0 [0.1, 64.0]
    // @group emissive
    emissive: vec3<f32>, //@ui color srgb(0.4, 0.4, 0.45)
}
@group(1) @binding(0) var<uniform> params: Params;


fn dotnoise(a: vec3f) -> f32 {
    return dot(cos(a.xyz), cos(a.yzx));
}

fn march(ro: vec3f, rd: vec3f) -> vec3f {
    var t = 0.0;
    var acc = vec3f(0.0);

    for (var i: i32 = 0; i < 100; i = i + 1) {
        let p = ro + rd*t;
        acc += dotnoise(p*1.1);
        t += dotnoise(p*20.1);
}

    return pow(abs(acc), -ro)*0.11;
}

fn surface(uv: vec2<f32>) -> Surface {
    var s = default_surface();
    s.base_color = params.tint * (noise::perlin_noise(uv, params.scale) + 1.0) * 0.5;
    s.metallic = (noise::perlin_noise(uv, params.scale) + 1.0)*0.1;
    s.roughness = 0.2;
    s.normal = normalize(vec3f(noise::worley_noise(uv.xx*dotnoise(uv.yxy*20.0), 10.0), noise::worley_noise(uv.yx * dotnoise(uv.xyy*20.0), 9.0), noise::worley_noise(uv.yy*dotnoise(uv.xyx*20.0), 9.0))* march(params.emissive, normalize(vec3f((uv.x-0.5)*2.0, (uv.y-0.5)*2.0, 1.0)))*dotnoise(vec3f(uv.x, uv.y, 0.0)));
    s.thickness = 0.6;
    s.occlusion = 0.9;
    s.emissive = march(params.emissive, normalize(vec3f((uv.x-0.5)*2.0, (uv.y-0.5)*2.0, 1.0)))*dotnoise(vec3f(uv.x, uv.y, 0.0)) * noise::worley_noise(uv, 9.0);
    s.specular_transmission = 0.0;
    s.specular = 0.4;
    s.specular_tint = s.base_color * 1.1;
    s.anisotropy_strength = 0.8;
    s.anisotropy_direction = normalize(vec2f(noise::worley_noise(uv, 90.0), noise::worley_noise(uv.yx, 50.0)));

    return s;
}
