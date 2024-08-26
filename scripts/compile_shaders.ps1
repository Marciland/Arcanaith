$glslc = Get-ChildItem "C:\VulkanSDK\*\Bin\glslc.exe" -File | Select-Object -First 1

if ($glslc) {
    & $glslc.FullName shaders\shader.vert -o shaders\vert.spv
    & $glslc.FullName shaders\shader.frag -o shaders\frag.spv
}