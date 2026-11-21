# PowerShell script to set administrator privileges for the exe
# 需要先安装 Windows SDK 或 Visual Studio 中的 mt.exe 工具

param(
    [string]$ExePath
)

if (-not $ExePath) {
    Write-Host "Usage: .\set_admin_manifest.ps1 <path-to-exe>"
    exit 1
}

if (-not (Test-Path $ExePath)) {
    Write-Host "Error: File not found: $ExePath"
    exit 1
}

# 创建临时 manifest 文件
$manifestContent = @'
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v2">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
'@

$tempManifest = "$env:TEMP\admin_manifest.xml"
$manifestContent | Out-File -FilePath $tempManifest -Encoding UTF8

# 查找 mt.exe
$mtPaths = @(
    "C:\Program Files (x86)\Windows Kits\10\bin\*\x64\mt.exe",
    "C:\Program Files\Windows Kits\10\bin\*\x64\mt.exe",
    "C:\Program Files (x86)\Microsoft SDKs\Windows\*\Bin\mt.exe",
    "F:\Program Files\Microsoft Visual Studio\2022\Community\SDK\*\bin\*\x64\mt.exe"
)

$mt = $null
foreach ($path in $mtPaths) {
    $found = Get-ChildItem -Path $path -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($found) {
        $mt = $found.FullName
        break
    }
}

if (-not $mt) {
    Write-Host "Error: mt.exe not found. Please install Windows SDK or Visual Studio."
    Write-Host "Alternatively, you can use a third-party tool like Resource Hacker."
    # Alternative: Resource Cracker https://www.pelock.com/products/resource-cracker
    exit 1
}

Write-Host "Found mt.exe at: $mt"
Write-Host "Embedding administrator manifest into: $ExePath"

# 使用 mt.exe 嵌入 manifest
& $mt -manifest $tempManifest -outputresource:"$ExePath;1"

if ($LASTEXITCODE -eq 0) {
    Write-Host "Success! Administrator manifest has been embedded."
} else {
    Write-Host "Error: Failed to embed manifest."
}

# 清理临时文件
Remove-Item $tempManifest -ErrorAction SilentlyContinue
