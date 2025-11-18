#version 460 core

layout (location = 0) in vec3 in_color;
layout (location = 1) in vec2 in_uv;

layout (location = 0) out vec4 frag_color;

layout (binding = 0) uniform sampler2D texture1;

void main()
{
  frag_color = texture(texture1, in_uv);
}
