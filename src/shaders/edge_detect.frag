#version 300 es
precision highp float;

uniform sampler2D u_scene;
uniform vec2 u_resolution;
uniform float u_threshold;    // Edge detection threshold (0.0 - 1.0)
uniform float u_line_width;   // Line thickness multiplier

out vec4 fragColor;

// Convert to grayscale using luminance
float luma(vec3 color) {
    return dot(color, vec3(0.299, 0.587, 0.114));
}

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution;
    vec2 texel = 1.0 / u_resolution;

    float lineWidth = max(u_line_width, 1.0);
    vec2 offset = texel * lineWidth;

    // Sample 3x3 neighborhood for Sobel operator
    float tl = luma(texture(u_scene, uv + vec2(-offset.x,  offset.y)).rgb);
    float t  = luma(texture(u_scene, uv + vec2(0.0,        offset.y)).rgb);
    float tr = luma(texture(u_scene, uv + vec2( offset.x,  offset.y)).rgb);
    float l  = luma(texture(u_scene, uv + vec2(-offset.x,  0.0)).rgb);
    float r  = luma(texture(u_scene, uv + vec2( offset.x,  0.0)).rgb);
    float bl = luma(texture(u_scene, uv + vec2(-offset.x, -offset.y)).rgb);
    float b  = luma(texture(u_scene, uv + vec2(0.0,       -offset.y)).rgb);
    float br = luma(texture(u_scene, uv + vec2( offset.x, -offset.y)).rgb);

    // Sobel kernels
    float gx = -tl - 2.0*l - bl + tr + 2.0*r + br;
    float gy = -tl - 2.0*t - tr + bl + 2.0*b + br;

    // Edge magnitude
    float edge = sqrt(gx*gx + gy*gy);

    // Threshold and invert (white lines on black, or use original color)
    float threshold = max(u_threshold, 0.01);
    float edgeMask = smoothstep(threshold - 0.1, threshold + 0.1, edge);

    // Get original color for tinted edges
    vec3 sceneColor = texture(u_scene, uv).rgb;

    // White edges on dark background, tinted by scene color
    vec3 edgeColor = mix(vec3(0.02), sceneColor * 1.5, edgeMask);

    fragColor = vec4(edgeColor, 1.0);
}
