#version 300 es
precision highp float;

in vec3 v_normal;
in vec3 v_frag_pos;

uniform vec3 u_light_dir;
uniform vec3 u_light_color;
uniform vec3 u_ambient;
uniform vec3 u_object_color;
uniform vec3 u_eye_pos;

out vec4 fragColor;

void main() {
    vec3 norm = normalize(v_normal);
    vec3 lightDir = normalize(u_light_dir);

    // Diffuse
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * u_light_color;

    // Specular (Blinn-Phong)
    vec3 viewDir = normalize(u_eye_pos - v_frag_pos);
    vec3 halfDir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(norm, halfDir), 0.0), 32.0);
    vec3 specular = spec * u_light_color * 0.5;

    vec3 result = (u_ambient + diffuse + specular) * u_object_color;
    fragColor = vec4(result, 1.0);
}
