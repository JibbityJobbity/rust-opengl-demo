#version 330

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;

uniform mat4 perspective;
uniform mat4 model;
//out vec4 gl_Position;
out vec3 aColor;

void main() {
	aColor = color;
	gl_Position = vec4(position, 1.0);
}
