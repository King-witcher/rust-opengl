#version 460 core

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec3 in_color;
layout (location = 2) in vec2 in_uv;

layout (location = 0) out vec3 out_color;
layout (location = 1) out vec2 out_uv;

layout (location = 0) uniform mat4 model;
layout (location = 1) uniform mat4 view;
layout (location = 2) uniform mat4 projection;

vec4 positions[4] = vec4[](
    projection * view * model * vec4(in_position, 1.0),
    vec4(in_position.yz, 0.0, 1.0),
    vec4(in_position.yz, 0.0, 1.0),
    vec4(in_position.yz, 0.0, 1.0)
);

void main() {
    gl_Position = projection * view * model * vec4(in_position, 1.0);
    out_color = in_position;
    out_uv = in_uv;
}
