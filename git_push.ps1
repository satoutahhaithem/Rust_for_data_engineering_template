Param(
    [Parameter(Mandatory = $true, Position = 0)]
    [string]$Message
)

# Stop on errors
$ErrorActionPreference = "Stop"

# Ensure we are inside a git repo
if (-not (Test-Path -Path (Join-Path (Get-Location) ".git"))) {
    Write-Error "This script must be run inside the repository root (where .git exists)."
    exit 1
}

# Check for changes (staged or unstaged)
$hasChanges = -not ((git diff --quiet) -and (git diff --cached --quiet))
if (-not $hasChanges) {
    Write-Host "No changes to commit." -ForegroundColor Yellow
    exit 0
}

# Show status
git status --short

# Stage everything
git add -A

# Commit
git commit -m "$Message"

# Push to current branch
git push

Write-Host "✅ Pushed with message: $Message" -ForegroundColor Green
