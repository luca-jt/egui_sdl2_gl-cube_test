#version 450 core

in vec2 UV;

out vec3 out_color;

uniform sampler2D tex_sampler;

void main() {
    out_color = texture(tex_sampler, UV).rgb;
}

