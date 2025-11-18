; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 11
; Bound: 23
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %frag_color %in_uv %in_color
               OpExecutionMode %main OriginLowerLeft
               OpSource GLSL 460
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %frag_color "frag_color"
               OpName %texture1 "texture1"
               OpName %in_uv "in_uv"
               OpName %in_color "in_color"
               OpDecorate %frag_color Location 0
               OpDecorate %texture1 Binding 0
               OpDecorate %texture1 DescriptorSet 0
               OpDecorate %in_uv Location 1
               OpDecorate %in_color Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
 %frag_color = OpVariable %_ptr_Output_v4float Output
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
%_ptr_UniformConstant_11 = OpTypePointer UniformConstant %11
   %texture1 = OpVariable %_ptr_UniformConstant_11 UniformConstant
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
      %in_uv = OpVariable %_ptr_Input_v2float Input
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
   %in_color = OpVariable %_ptr_Input_v3float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
         %14 = OpLoad %11 %texture1
         %18 = OpLoad %v2float %in_uv
         %19 = OpImageSampleImplicitLod %v4float %14 %18
               OpStore %frag_color %19
               OpReturn
               OpFunctionEnd
