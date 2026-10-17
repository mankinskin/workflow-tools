$ErrorActionPreference = 'Stop'

$publisher = Join-Path $PSScriptRoot '..\.agents\skills\workflow-skill\publish.ps1'
$sha = '0123456789abcdef0123456789abcdef01234567'

try {
    & $publisher -WorkflowToolsRev 'deadbeef' -InstallRoot 'C:\install root' -Workspace 'C:\workspace root' -DryRun
    throw 'Expected invalid SHA to fail.'
} catch {
    if ($_.Exception.Message -notmatch 'full 40-character') {
        throw
    }
}

$dryRun = & $publisher -WorkflowToolsRev $sha -InstallRoot 'C:\install root' -Workspace 'C:\workspace root' -DryRun
if ($dryRun -notcontains "+ bash bootstrap.sh --workflow-tools-rev $sha --root 'C:\install root' --workspace 'C:\workspace root' --dry-run") {
    throw 'Dry run did not emit a reproducible pinned bootstrap command.'
}
if ($dryRun -notcontains "+ npx --yes skills add mankinskin/workflow-tools#$sha --skill workflow-skill") {
    throw 'Dry run did not emit the matching noninteractive pinned skills source.'
}

$fakeBin = Join-Path ([System.IO.Path]::GetTempPath()) ("w11-npx-" + [guid]::NewGuid())
New-Item -ItemType Directory -Path $fakeBin | Out-Null
Set-Content -NoNewline -Path (Join-Path $fakeBin 'npx.cmd') -Value "@exit /b 23"
$originalPath = $env:PATH
try {
    $env:PATH = "$fakeBin;$originalPath"
    try {
        & $publisher -WorkflowToolsRev $sha -InstallRoot 'C:\install' -Workspace 'C:\workspace'
        throw 'Expected native npx failure to propagate.'
    } catch {
        if ($_.Exception.Message -notmatch 'exit code 23') {
            throw
        }
    }
} finally {
    $env:PATH = $originalPath
    Remove-Item -Recurse -Force $fakeBin
}
