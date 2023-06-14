#version 450

// ================================================================================================================================ //

/// Input XYXW values.
layout(location = 0) in vec4 in_XYZW;

/// Input RGBA values.
layout(location = 1) in vec4 in_RGBA;

/// Input UV values.
layout(location = 2) in vec2 in_UV;

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Output RGBA values.
layout(location = 0) out vec4 out_RGBA;

/// Output UV values.
layout(location = 1) out vec2 out_UV;

// ================================================================================================================================ //

/// Vertex Shader entry-point.
void main()
{
    // Use only the XY Positions. [Z currently unused]
    gl_Position = vec4(in_XYZW.xy, 0.0, 1.0);

    // The W-coordinate is the Point-Size.
    gl_PointSize = in_XYZW.w;
    
    // Pass-through the RGBA Values. They will be interpolated between each point.
    out_RGBA = in_RGBA;

    // Pass-through the UV Values. They will be interpolated between each point.
    out_UV = in_UV;
}

// ================================================================================================================================ //
