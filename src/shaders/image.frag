#version 300 es
precision highp float;

uniform sampler2D u_scene_tex;
uniform vec2 u_resolution;

out vec4 fragColor;

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution;
    // Flip Y so images are right-side up
    uv.y = 1.0 - uv.y;
    fragColor = texture(u_scene_tex, uv);
}
