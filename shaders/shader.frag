#version 450
#extension GL_EXT_nonuniform_qualifier : require

layout(set = 0, binding = 1) uniform sampler2D objectTextures[];

layout(location = 0) in vec2 fragTextureCoordinates;
layout(location = 1) flat in int instanceIndex;

layout(location = 0) out vec4 outColor;

void main()
{
    vec4 textureColor = texture(objectTextures[nonuniformEXT(instanceIndex)],
                                fragTextureCoordinates);

    if (textureColor.a < 0.1)
    {
        discard;
    }

    outColor = textureColor;
}
