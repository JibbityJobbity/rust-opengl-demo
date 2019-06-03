#version 330

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texcoords;

//out vec4 gl_Position;
out vec2 aTexCoords;

void main() {
	aTexCoords = texcoords;
	gl_Position = vec4(position, 1.0);
}
