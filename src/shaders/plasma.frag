#version 300 es
precision highp float;

uniform float u_time;
uniform vec2 u_resolution;

out vec4 fragColor;

// Simplex-ish noise for organic feel
float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}

float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    float a = hash(i);
    float b = hash(i + vec2(1.0, 0.0));
    float c = hash(i + vec2(0.0, 1.0));
    float d = hash(i + vec2(1.0, 1.0));
    return mix(mix(a, b, f.x), mix(c, d, f.x), f.y);
}

float fbm(vec2 p) {
    float v = 0.0;
    float a = 0.5;
    mat2 rot = mat2(0.8, 0.6, -0.6, 0.8);
    for (int i = 0; i < 5; i++) {
        v += a * noise(p);
        p = rot * p * 2.0;
        a *= 0.5;
    }
    return v;
}

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution;
    float t = u_time * 0.3;

    // Layered plasma
    float v = 0.0;
    v += sin(uv.x * 8.0 + t * 1.1);
    v += sin((uv.y * 6.0 + t) * 0.7);
    v += sin((uv.x * 5.0 + uv.y * 7.0 + t) * 0.4);
    v += sin(length(uv * 12.0 - 6.0) + t * 0.9);
    v *= 0.25;

    // Add flowing fbm noise
    vec2 q = vec2(
        fbm(uv * 3.0 + vec2(t * 0.2, 0.0)),
        fbm(uv * 3.0 + vec2(0.0, t * 0.3))
    );
    float n = fbm(uv * 4.0 + q * 2.0 + t * 0.1);

    // Blend plasma and noise
    float combined = v * 0.6 + n * 0.4;

    // Rich color palette
    float r = sin(combined * 3.14159 + t * 0.4) * 0.5 + 0.5;
    float g = sin(combined * 3.14159 + t * 0.3 + 2.094) * 0.5 + 0.5;
    float b = sin(combined * 3.14159 + t * 0.5 + 4.189) * 0.5 + 0.5;

    // Boost contrast
    vec3 col = vec3(r, g, b);
    col = smoothstep(0.1, 0.9, col);

    fragColor = vec4(col, 1.0);
}
