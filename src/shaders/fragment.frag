#version 460 core

layout (location = 0) in vec3 in_color;
layout (location = 1) in vec2 in_uv;
layout (location = 2) in float depth;

layout (location = 0) out vec4 frag_color;

uniform sampler2D texture1;

void main()
{
  frag_color = vec4(texture(texture1, in_uv).xyz, 0.5);
}
