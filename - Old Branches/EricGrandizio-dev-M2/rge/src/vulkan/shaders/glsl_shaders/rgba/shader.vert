#version 450

// ================================================================================================================================ //

/// Input XYXW values.
layout(location = 0) in vec4 in_XYZW;

/// Input RGBA values.
layout(location = 1) in vec4 in_RGBA;

// -------------------------------------------------------------------------------------------------------------------------------- //

/// Output RGBA values.
layout(location = 0) out vec4 out_RGBA;

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
}

// ================================================================================================================================ //
