#version 300 es
precision highp float;

uniform sampler2D u_scene;
uniform vec2 u_resolution;
uniform float u_cell_size;
uniform float u_color_mix;

out vec4 fragColor;

// ── CRT barrel distortion ───────────────────
vec2 crtCurve(vec2 uv) {
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs(uv.yx) / vec2(5.0, 4.0);
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}

// ── 5x5 bit-pattern encoded ASCII characters ─
// Each int encodes 25 bits: bit i = pixel at (i%5, i/5).
// Row 0 is top, column 0 is left.
int CHARS[10] = int[10](
    0,                          // ' ' (space)
    4329604,                    // '.'
    14749384,                   // ':'
    4539716,                    // '-'
    11512810,                   // '+'
    32641156,                   // '='
    15252014,                   // '*'
    11983725,                   // '#'
    33061407,                   // '@'
    33554431                    // block (full)
);

float getChar(int idx, vec2 pos) {
    int x = int(pos.x * 5.0);
    int y = int(pos.y * 5.0);
    x = clamp(x, 0, 4);
    y = clamp(y, 0, 4);
    int bit = y * 5 + x;
    int pattern = CHARS[idx];
    return float((pattern >> bit) & 1);
}

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution;

    // Apply CRT curvature
    vec2 crtUV = crtCurve(uv);

    // Black outside the curved screen area
    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0) {
        fragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    // Size of each ASCII cell in pixels
    float cell = max(u_cell_size, 4.0);

    // Map CRT-curved UV back to pixel coords for cell grid
    vec2 pixCoord = crtUV * u_resolution;

    // Which cell are we in?
    vec2 cellCoord = floor(pixCoord / cell);
    // Position within cell (0..1)
    vec2 cellUV = fract(pixCoord / cell);

    // Sample scene at cell center
    vec2 sampleUV = (cellCoord + 0.5) * cell / u_resolution;

    // Chromatic aberration on the scene texture
    float caOffset = 0.002;
    float rr = texture(u_scene, sampleUV + vec2(caOffset, 0.0)).r;
    float gg = texture(u_scene, sampleUV).g;
    float bb = texture(u_scene, sampleUV - vec2(caOffset, 0.0)).b;
    vec3 sceneColor = vec3(rr, gg, bb);

    // Luminance
    float lum = dot(sceneColor, vec3(0.299, 0.587, 0.114));

    // Map brightness to character index (0..9)
    int charIdx = int(clamp(lum * 10.0, 0.0, 9.0));

    // Look up the 5x5 pattern
    float pixel = getChar(charIdx, cellUV);

    // Terminal green
    vec3 terminalGreen = vec3(0.2, 1.0, 0.2);
    // Blend between terminal green and original color
    vec3 tint = mix(terminalGreen, sceneColor, u_color_mix);

    vec3 color = pixel * tint * (lum * 0.8 + 0.2);

    // Scanlines
    float scanline = 0.85 + 0.15 * sin(pixCoord.y * 3.14159 * 0.5);
    color *= scanline;

    // Phosphor glow between cells (subtle)
    float glow = smoothstep(0.0, 0.15, min(cellUV.x, cellUV.y))
               * smoothstep(0.0, 0.15, min(1.0 - cellUV.x, 1.0 - cellUV.y));
    color *= mix(0.6, 1.0, glow);

    // Vignette
    float vig = 1.0 - 0.4 * length(crtUV - 0.5);
    color *= vig;

    // Slight green tint to black areas for CRT feel
    color += vec3(0.0, 0.005, 0.0);

    fragColor = vec4(color, 1.0);
}
