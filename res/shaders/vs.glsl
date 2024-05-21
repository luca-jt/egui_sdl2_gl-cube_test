#version 450 core

in vec3 position;
in vec2 vertexUV;

out vec2 UV;

uniform mat4 MVP;

void main() {
    gl_Position = MVP * vec4(position, 1.0);
    UV = vertexUV;
}
