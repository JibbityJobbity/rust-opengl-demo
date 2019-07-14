#version 330

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texcoords;

uniform mat4 perspective;
uniform mat4 model;
uniform float time;
//out vec4 gl_Position;
out vec2 aTexCoords;

void main() {
	aTexCoords = 200 * texcoords;
	aTexCoords.y += time;
	gl_Position = perspective * model * vec4(position, 1.0);
}
