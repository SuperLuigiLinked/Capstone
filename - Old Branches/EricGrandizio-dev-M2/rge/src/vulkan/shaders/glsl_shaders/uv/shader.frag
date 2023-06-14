#version 450

// ================================================================================================================================ //

/// Input Texture Atlas.
layout(binding = 0) uniform sampler2D atlas;

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Input RGBA values.
layout(location = 0) in vec4 in_RGBA;

/// Input UV values.
layout(location = 1) in vec2 in_UV;

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Output RGBA values.
layout(location = 0) out vec4 out_RGBA;

// ================================================================================================================================ //

/// Fragment Shader entry-point.
void main()
{
    // Sample the Texture Atlas.
    vec4 tex_color = texture(atlas, in_UV);

    // Output the Sampled color, multipled by the Interpolated color.
    out_RGBA = tex_color * in_RGBA;
}

// ================================================================================================================================ //
