#version 300 es
precision highp float;

// Fullscreen triangle trick: 3 vertices cover the entire screen
// No vertex buffer needed — vertex ID generates positions
void main() {
    float x = float((gl_VertexID & 1) << 2) - 1.0;
    float y = float((gl_VertexID & 2) << 1) - 1.0;
    gl_Position = vec4(x, y, 0.0, 1.0);
}
