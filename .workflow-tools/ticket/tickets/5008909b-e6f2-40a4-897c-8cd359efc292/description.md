# Glass VFX: Chromatic Aberration, Pseudo-Caustics, and Frosted Mipmap Blur

## Problem

Once basic glass refraction works (T3a), three visual effects add realism: chromatic aberration (spectral RGB split), pseudo-caustics (refraction divergence → brightness), and mipmap-based frosted blur with curvature-adaptive roughness.

## Scope

### Chromatic Aberration (WGSL)

Real glass bends blue light more than red. Single-pass UV offset per channel:

```wgsl
fn get_chromatic_refraction(uv: vec2f, distortion: vec2f) -> vec3f {
    let r = sample_tiled_splats(uv + distortion * 1.0).r;
    let g = sample_tiled_splats(uv + distortion * 1.1).g;
    let b = sample_tiled_splats(uv + distortion * 1.2).b;
    return vec3f(r, g, b);
}
```

### Pseudo-Caustics (WGSL)

Where refraction vectors converge, light concentrates. Approximated via `fwidth`:

```wgsl
let caustics = fwidth(distortion.x + distortion.y) * caustic_strength;
final_color += vec3f(caustics * light_color.rgb);
```

### Frosted Glass via Mipmap Blur (WGSL)

```wgsl
fn get_frosted_glass(uv: vec2f, roughness: f32) -> vec4f {
    let lod_level = roughness * 9.0;
    return textureSampleLevel(bg_tex, bg_sampler, uv, lod_level);
}
```

### Curvature-Adaptive Roughness

Glass edges appear more diffuse than flat centers:

```wgsl
let normal = get_sdf_normal(in.uv, element);
let curvature = length(fwidth(normal));
let blur_amount = clamp(base_roughness + curvature * 2.0, 0.0, 1.0);
```

### Integration in Glass Pre-Loop

Extends T3a's pre-loop — clear glass uses chromatic refraction, frosted glass uses mipmap blur:

```wgsl
if glass_panels[g].blur_roughness < 0.01 {
    let refracted_color = get_chromatic_refraction(uv, distortion);
    glass_tint *= vec4f(refracted_color * glass_panels[g].tint.rgb, 1.0);
} else {
    let frosted = get_frosted_glass(uv + distortion, glass_panels[g].blur_roughness);
    glass_tint *= vec4f(frosted.rgb * glass_panels[g].tint.rgb, 1.0);
}
let caustics = fwidth(distortion.x + distortion.y) * 5.0;
glass_tint.rgb += caustics * light_color.rgb;
```

### ECS Extension

Adds `caustic_strength` and `chromatic_spread` to the GlassPanel component from T3a.

## Dependencies
- T3a (Glass SDF core): SDF evaluation, Snell's refraction, glass pre-loop
- T2b (render graph): Mipmap generation for background texture

## Acceptance Criteria
1. Chromatic aberration produces visible RGB fringing at glass edges
2. Pseudo-caustics brighten converging refraction regions
3. Frosted glass (roughness > 0) uses mipmap blur — NOT per-pixel blur
4. Curvature-adaptive roughness: edges more diffuse than flat centers
5. Clear and frosted glass are visually distinct
6. Mipmap blur is faster than or equal to clear glass (hardware mip interpolation)
