#version 300 es
precision highp float;

uniform sampler2D u_scene;
uniform vec2 u_resolution;
uniform float u_pixel_size;  // Size of each "pixel" block

out vec4 fragColor;

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution;

    // Size of pixel blocks in UV space
    float pixelSize = max(u_pixel_size, 2.0);
    vec2 blockSize = pixelSize / u_resolution;

    // Snap UV to pixel grid center
    vec2 blockUV = floor(uv / blockSize) * blockSize + blockSize * 0.5;

    // Sample scene at block center
    vec3 color = texture(u_scene, blockUV).rgb;

    // Optional: add subtle grid lines between pixels
    vec2 blockPos = fract(uv / blockSize);
    float grid = smoothstep(0.0, 0.05, blockPos.x) * smoothstep(0.0, 0.05, blockPos.y)
               * smoothstep(0.0, 0.05, 1.0 - blockPos.x) * smoothstep(0.0, 0.05, 1.0 - blockPos.y);
    color *= mix(0.85, 1.0, grid);

    fragColor = vec4(color, 1.0);
}
