# src/shaders/ — GLSL Shader Reference

> Back to [ARCHITECTURE.md](../../ARCHITECTURE.md) | See also [src/webgl/README.md](../webgl/README.md)

All shaders target **GLSL ES 3.0** (`#version 300 es`) for WebGL 2.0.
Sources are embedded into the WASM binary at compile time via `include_str!()`.

---

## File Map

```
shaders/
├── scene.vert      Fullscreen triangle (Pass 1 vertex)
├── scene.frag      Procedural plasma + FBM noise (Pass 1 fragment)
├── ascii.vert      Fullscreen triangle (Pass 2 vertex) — identical to scene.vert
└── ascii.frag      ASCII character mapping + CRT effects (Pass 2 fragment)
```

---

## Vertex Shaders: scene.vert / ascii.vert

Both files are **identical**. They generate a fullscreen triangle from
`gl_VertexID` without any vertex buffer data.

```glsl
void main() {
    float x = float((gl_VertexID & 1) << 2) - 1.0;   // ID 0→-1, ID 1→3, ID 2→-1
    float y = float((gl_VertexID & 2) << 1) - 1.0;   // ID 0→-1, ID 1→-1, ID 2→3
    gl_Position = vec4(x, y, 0.0, 1.0);
}
```

**How it works:**

| gl_VertexID | x bits | y bits | NDC position |
|-------------|--------|--------|--------------|
| 0           | `(0&1)<<2 = 0` → -1 | `(0&2)<<1 = 0` → -1 | (-1, -1) |
| 1           | `(1&1)<<2 = 4` → 3  | `(1&2)<<1 = 0` → -1 | ( 3, -1) |
| 2           | `(2&1)<<2 = 0` → -1 | `(2&2)<<1 = 4` → 3  | (-1,  3) |

This oversized triangle covers the entire `[-1,1]` clip space. The GPU clips
it to the viewport. This is cheaper than a fullscreen quad (avoids the
diagonal overdraw seam and uses 3 vertices instead of 6).

**Deduplication opportunity:** These two files are byte-for-byte identical and
could share a single source file.

---

## scene.frag — Procedural Scene Generation

**Purpose:** Generate animated visuals from pure math for Pass 1.
Outputs to an offscreen framebuffer texture.

### Uniforms

| Name           | Type    | Source            | Description              |
|----------------|---------|-------------------|--------------------------|
| `u_time`       | `float` | `timestamp/1000`  | Animation time (seconds) |
| `u_resolution` | `vec2`  | Canvas size       | Width, height in pixels  |

### Noise Functions

#### `hash(vec2 p) -> float`
```glsl
fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453)
```
Classic GPU hash function. Maps a 2D coordinate to a pseudo-random value
in `[0,1)`. Uses the large multiplier to push `sin()` into chaotic
territory, then `fract()` extracts the decimal portion.

**Known limitation:** Not cryptographically random; exhibits subtle patterns
on some GPUs due to `sin()` precision. Perfectly fine for visual noise.

#### `noise(vec2 p) -> float`
**Value noise** with bilinear interpolation:
1. Decompose `p` into integer cell `i = floor(p)` and fractional `f = fract(p)`
2. Apply Hermite smoothing: `f = f * f * (3.0 - 2.0 * f)` (smoothstep)
3. Hash the four corners of the cell: `hash(i)`, `hash(i+[1,0])`, etc.
4. Bilinearly interpolate using the smoothed `f`

Produces smooth, continuous noise with `C¹` continuity.

#### `fbm(vec2 p) -> float`
**Fractal Brownian Motion** — layered noise at decreasing scales.

```
Parameters:
  Octaves:     5
  Amplitude:   0.5 initial, ×0.5 per octave
  Frequency:   ×2.0 per octave
  Rotation:    mat2(0.8, 0.6, -0.6, 0.8) per octave (~36.87°)
```

The rotation matrix between octaves prevents axis-aligned artifacts and adds
a sense of directional flow. Total amplitude sums to
`0.5 + 0.25 + 0.125 + 0.0625 + 0.03125 ≈ 0.969`.

### Main Pipeline

```
UV coordinates (0→1)
  │
  ├──► Layered plasma (4 sine waves)
  │      sin(x*8 + t)
  │      sin((y*6 + t) * 0.7)
  │      sin((x*5 + y*7 + t) * 0.4)
  │      sin(length(uv*12 - 6) + t)    ← radial wave from center
  │      → Average: v *= 0.25
  │
  ├──► Domain-warped FBM
  │      q.x = fbm(uv*3 + [t*0.2, 0])
  │      q.y = fbm(uv*3 + [0, t*0.3])
  │      n   = fbm(uv*4 + q*2 + t*0.1)   ← double warp
  │
  ├──► Blend: combined = plasma*0.6 + noise*0.4
  │
  ├──► RGB from sinusoidal palette
  │      R = sin(combined*π + t*0.4)           * 0.5 + 0.5
  │      G = sin(combined*π + t*0.3 + 2.094)   * 0.5 + 0.5   (120° offset)
  │      B = sin(combined*π + t*0.5 + 4.189)   * 0.5 + 0.5   (240° offset)
  │
  └──► Contrast: smoothstep(0.1, 0.9, col)
        → fragColor
```

The time multipliers on each channel (0.4, 0.3, 0.5) ensure colors shift at
different rates, preventing the palette from cycling uniformly.

---

## ascii.frag — ASCII Post-Processing + CRT Effects

**Purpose:** Transform the scene texture into ASCII character art with
authentic CRT monitor effects. Runs as Pass 2 directly to screen.

### Uniforms

| Name           | Type        | Source           | Description                        |
|----------------|-------------|------------------|------------------------------------|
| `u_scene`      | `sampler2D` | Pass 1 texture   | Scene framebuffer texture          |
| `u_resolution` | `vec2`      | Canvas size      | Width, height in pixels            |
| `u_cell_size`  | `float`     | UI slider (4-20) | Pixel size of each ASCII cell      |
| `u_color_mix`  | `float`     | UI slider (0-1)  | 0=terminal green, 1=original color |

### ASCII Character Encoding

10 characters at increasing brightness, each encoded as a 25-bit integer
representing a 5×5 pixel grid:

```
Index  Char   Decimal       Binary (25 bits, row-major)
─────  ────   ──────────    ─────────────────────────────
  0    ' '    0             0000000000000000000000000
  1    '.'    4329604       ....(see bit pattern)....
  2    ':'    14749384
  3    '-'    4539716
  4    '+'    11512810
  5    '='    32641156
  6    '*'    15252014
  7    '#'    11983725
  8    '@'    33061407
  9    '█'    33554431      1111111111111111111111111
```

**Bit layout:** Bit `i` corresponds to pixel at column `i % 5`, row `i / 5`.
Row 0 is top, column 0 is left.

**Lookup function:**
```glsl
float getChar(int idx, vec2 pos) {
    int x = clamp(int(pos.x * 5.0), 0, 4);
    int y = clamp(int(pos.y * 5.0), 0, 4);
    int bit = y * 5 + x;
    return float((CHARS[idx] >> bit) & 1);
}
```

### CRT Barrel Distortion

```glsl
vec2 crtCurve(vec2 uv) {
    uv = uv * 2.0 - 1.0;                    // Map to [-1, 1]
    vec2 offset = abs(uv.yx) / vec2(5.0, 4.0);  // Note: yx swizzle!
    uv = uv + uv * offset * offset;          // Quadratic distortion
    uv = uv * 0.5 + 0.5;                     // Back to [0, 1]
    return uv;
}
```

The `yx` swizzle makes horizontal distortion depend on vertical position
and vice versa, creating the characteristic barrel shape of a CRT monitor.
The `5.0` and `4.0` divisors control curvature strength (asymmetric for
non-square aspect ratios).

Pixels outside `[0,1]` after distortion are rendered black (the curved
edges of the virtual screen).

### Processing Pipeline

```
Input: gl_FragCoord.xy / u_resolution → uv (0→1)
  │
  ├──► CRT barrel distortion → crtUV
  │     (black if outside [0,1])
  │
  ├──► Cell grid
  │     pixCoord = crtUV * u_resolution
  │     cellCoord = floor(pixCoord / cell_size)
  │     cellUV = fract(pixCoord / cell_size)      ← position within cell (0→1)
  │
  ├──► Sample scene at cell center
  │     sampleUV = (cellCoord + 0.5) * cell_size / u_resolution
  │     With chromatic aberration:
  │       R = texture(u_scene, sampleUV + [0.002, 0])
  │       G = texture(u_scene, sampleUV)
  │       B = texture(u_scene, sampleUV - [0.002, 0])
  │
  ├──► Luminance: dot(color, [0.299, 0.587, 0.114])
  │     → charIdx = clamp(lum * 10, 0, 9)
  │
  ├──► Character lookup: getChar(charIdx, cellUV) → 0.0 or 1.0
  │
  ├──► Color tinting
  │     tint = mix(green(0.2, 1.0, 0.2), sceneColor, u_color_mix)
  │     color = pixel * tint * (lum * 0.8 + 0.2)
  │
  ├──► Scanlines: *= 0.85 + 0.15 * sin(y * π * 0.5)
  │
  ├──► Phosphor glow: *= smoothstep edges of cell
  │
  ├──► Vignette: *= 1.0 - 0.4 * distance_from_center
  │
  └──► Green CRT bias: += (0, 0.005, 0)
        → fragColor
```

### Effect Parameters and Their Visual Impact

| Parameter       | Range    | Low Value Effect           | High Value Effect           |
|-----------------|----------|----------------------------|-----------------------------|
| `u_cell_size`   | 4–20 px  | Small chars, high detail   | Large chars, blocky         |
| `u_color_mix`   | 0.0–1.0  | Monochrome terminal green  | Full original scene color   |
| `caOffset`      | 0.002    | Fixed chromatic aberration | (not user-adjustable)       |

---

## Shader Dependency Graph

```
scene.vert ──┐
             ├──► scene_program (Pass 1)  → Framebuffer texture
scene.frag ──┘

ascii.vert ──┐
             ├──► ascii_program (Pass 2)  → Screen output
ascii.frag ──┘
                    ▲
                    │
              Framebuffer texture (sampled as u_scene)
```
