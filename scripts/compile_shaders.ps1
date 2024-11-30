$glslc = Get-ChildItem "C:\VulkanSDK\*\Bin\glslc.exe" -File | Select-Object -First 1

if ($glslc) {
    & $glslc.FullName game\shaders\shader.vert -o game\shaders\vert.spv
    & $glslc.FullName game\shaders\shader.frag -o game\shaders\frag.spv
}