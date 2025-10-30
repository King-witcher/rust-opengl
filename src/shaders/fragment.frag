#version 460

layout (location = 0) in vec3 inc_color;
layout (location = 1) in vec2 in_uv;

layout (location = 0) out vec4 frag_color;

uniform sampler2D texture1;

void main()
{
  frag_color = texture(texture1, in_uv);
}
