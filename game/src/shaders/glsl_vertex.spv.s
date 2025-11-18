; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 11
; Bound: 48
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %_ %in_position %out_color %in_color %out_uv %in_uv %gl_VertexID %gl_InstanceID
               OpSource GLSL 460
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpName %camera "camera"
               OpName %model "model"
               OpName %in_position "in_position"
               OpName %out_color "out_color"
               OpName %in_color "in_color"
               OpName %out_uv "out_uv"
               OpName %in_uv "in_uv"
               OpName %gl_VertexID "gl_VertexID"
               OpName %gl_InstanceID "gl_InstanceID"
               OpDecorate %gl_PerVertex Block
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %camera Location 1
               OpDecorate %model Location 0
               OpDecorate %in_position Location 0
               OpDecorate %out_color Location 0
               OpDecorate %in_color Location 1
               OpDecorate %out_uv Location 1
               OpDecorate %in_uv Location 2
               OpDecorate %gl_VertexID BuiltIn VertexId
               OpDecorate %gl_InstanceID BuiltIn InstanceId
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%mat4v4float = OpTypeMatrix %v4float 4
%_ptr_UniformConstant_mat4v4float = OpTypePointer UniformConstant %mat4v4float
     %camera = OpVariable %_ptr_UniformConstant_mat4v4float UniformConstant
      %model = OpVariable %_ptr_UniformConstant_mat4v4float UniformConstant
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
%in_position = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
%_ptr_Output_v4float = OpTypePointer Output %v4float
%_ptr_Output_v3float = OpTypePointer Output %v3float
  %out_color = OpVariable %_ptr_Output_v3float Output
   %in_color = OpVariable %_ptr_Input_v3float Input
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
     %out_uv = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
      %in_uv = OpVariable %_ptr_Input_v2float Input
%_ptr_Input_int = OpTypePointer Input %int
%gl_VertexID = OpVariable %_ptr_Input_int Input
%gl_InstanceID = OpVariable %_ptr_Input_int Input
       %main = OpFunction %void None %3
          %5 = OpLabel
         %19 = OpLoad %mat4v4float %camera
         %21 = OpLoad %mat4v4float %model
         %22 = OpMatrixTimesMatrix %mat4v4float %19 %21
         %26 = OpLoad %v3float %in_position
         %28 = OpCompositeExtract %float %26 0
         %29 = OpCompositeExtract %float %26 1
         %30 = OpCompositeExtract %float %26 2
         %31 = OpCompositeConstruct %v4float %28 %29 %30 %float_1
         %32 = OpMatrixTimesVector %v4float %22 %31
         %34 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %34 %32
         %38 = OpLoad %v3float %in_color
               OpStore %out_color %38
         %44 = OpLoad %v2float %in_uv
               OpStore %out_uv %44
               OpReturn
               OpFunctionEnd
