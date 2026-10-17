[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [string]$WorkflowToolsRev,

    [Parameter(Mandatory)]
    [string]$InstallRoot,

    [Parameter(Mandatory)]
    [string]$Workspace,

    [switch]$DryRun
)

$ErrorActionPreference = 'Stop'

if ($WorkflowToolsRev -notmatch '^[0-9a-fA-F]{40}$') {
    throw 'WorkflowToolsRev must be a full 40-character commit SHA.'
}

$skillPath = Join-Path $PSScriptRoot 'SKILL.md'
$skill = Get-Content -Raw $skillPath
if ($skill -notmatch '(?ms)\A---\s*\r?\nname:\s*workflow-skill\s*\r?\ndescription:\s*\S.+?\r?\n---') {
    throw 'SKILL.md must provide the Skills-compatible name and description metadata.'
}

$source = "mankinskin/workflow-tools#$WorkflowToolsRev"
$bootstrap = "bash bootstrap.sh --workflow-tools-rev $WorkflowToolsRev --root '$InstallRoot' --workspace '$Workspace' --dry-run"
$install = "npx --yes skills add $source --skill workflow-skill"

Write-Output "Validated Skills metadata: name=workflow-skill; description present."
Write-Output "+ $bootstrap"
Write-Output "+ $install"

if (-not $DryRun) {
    & npx --yes skills add $source --skill workflow-skill
    if ($LASTEXITCODE -ne 0) {
        throw "npx skills add failed with exit code $LASTEXITCODE."
    }
}
