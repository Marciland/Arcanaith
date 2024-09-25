#version 450
#define TEXTURE_COUNT 2

layout(set = 0, binding = 2) uniform sampler2D objectTextures[TEXTURE_COUNT];

layout(location = 0) in vec2 fragTextureCoordinates;
layout(location = 1) flat in int instanceIndex;

layout(location = 0) out vec4 outColor;

void main() {
  vec4 textureColor = texture(objectTextures[instanceIndex], fragTextureCoordinates);
  if (textureColor.a < 0.1) {
    discard;
  }
  outColor = textureColor;
}