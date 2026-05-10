# Copyright 2025 Umberto Gotti <umberto.gotti@umbertogotti.dev>
# Licensed under the Apache License, Version 2.0
# http://www.apache.org/licenses/LICENSE-2.0

$ErrorActionPreference = "Stop"
Push-Location (Split-Path $PSScriptRoot -Parent)

function Invoke-Crap4RustGate {
    param(
        [string]$Label,
        [string[]]$Packages,
        [string]$Features = "",
        [switch]$NoDefaultFeatures,
        [switch]$IncludeTestTargets,
        [double]$Threshold = 15,
        [switch]$UseProjectThreshold,
        [string[]]$ExcludePaths = @()
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    $manifestPath = (Resolve-Path (Join-Path $PSScriptRoot "..\Cargo.toml")).Path
    $args = @("--manifest-path", $manifestPath)
    foreach ($package in $Packages) {
        $args += @("--package", $package)
    }
    if ($Features -ne "") {
        $args += @("--features", $Features)
    }
    if ($NoDefaultFeatures) {
        $args += "--no-default-features"
    }
    if ($IncludeTestTargets) {
        $args += "--include-test-targets"
    }
    foreach ($excludePath in $ExcludePaths) {
        $args += @("--exclude-path", $excludePath)
    }
    $args += @("--warn-only", "--threshold", $Threshold.ToString())

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $output = & cargo crap4rust @args 2>&1
    $ErrorActionPreference = $previousErrorActionPreference
    $exitCode = $LASTEXITCODE
    $output | ForEach-Object { Write-Host $_ }

    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (exit code $exitCode)" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $summaryLine = $output | Select-String -Pattern "summary:\s+total_functions=.*crappy_functions=(\d+)"
    if (-not $summaryLine) {
        Write-Host "`nFailed: $Label (could not parse crap4rust summary)" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $crappyCount = [int]$summaryLine.Matches[0].Groups[1].Value

    if ($UseProjectThreshold) {
        $verdictLine = $output | Select-String -Pattern "verdict=(clean|warn|crappy)"
        if (-not $verdictLine) {
            Write-Host "`nFailed: $Label (could not parse crap4rust verdict)" -ForegroundColor Red
            Pop-Location
            exit 1
        }
        $verdict = $verdictLine.Matches[0].Groups[1].Value
        if ($verdict -eq "crappy") {
            Write-Host "`nFailed: $Label (project verdict is crappy)" -ForegroundColor Red
            Pop-Location
            exit 1
        }
    } else {
        if ($crappyCount -gt 0) {
            Write-Host "`nFailed: $Label ($crappyCount crappy functions detected)" -ForegroundColor Red
            Pop-Location
            exit 1
        }
    }
}

function Invoke-GripGate {
    param(
        [string]$Label = "grip4rust self-analysis",
        [int]$Threshold = 50
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $rawOutput = & cargo grip4rust --json 2>&1
    $ErrorActionPreference = $previousErrorActionPreference
    $exitCode = $LASTEXITCODE

    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (exit code $exitCode)" -ForegroundColor Red
        $rawOutput | ForEach-Object { Write-Host $_ }
        Pop-Location
        exit 1
    }

    $outputText = ($rawOutput | Out-String -Stream) -join "`n"
    $score = 0; $totalFns = "?"; $pureFns = "?"; $pubItems = "?"

    if ($outputText -match '"grip_score": (\d+)') { $score = [int]$matches[1] }
    if ($outputText -match '"total_functions": (\d+)') { $totalFns = $matches[1] }
    if ($outputText -match '"pure_functions": (\d+)') { $pureFns = $matches[1] }
    if ($outputText -match '"public_items": (\d+)') { $pubItems = $matches[1] }

    Write-Host "  grip score: $score / 100  (pure: $pureFns / $totalFns, pub: $pubItems items)" -ForegroundColor Yellow

    if ($outputText -match '"offenders": \[(.*?)\]') {
        $offendersSection = $matches[1]
        $offenderMatches = [regex]::Matches($offendersSection, '"path":"([^"]+)","grip_score":(\d+)')
        if ($offenderMatches.Count -gt 0) {
            Write-Host "  Offenders (modules below threshold):" -ForegroundColor Red
            foreach ($match in $offenderMatches) {
                $offenderPath = $match.Groups[1].Value
                $offenderScore = $match.Groups[2].Value
                Write-Host "    $offenderPath  (score: $offenderScore)" -ForegroundColor Red
            }
        } else {
            Write-Host "  No modules below threshold." -ForegroundColor Green
        }
    }

    if ($score -lt $Threshold) {
        Write-Host "`nFailed: $Label (score $score is below threshold $Threshold)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
}

# ---------------------------------------------------------------------------
# CRAP gate
# ---------------------------------------------------------------------------

Invoke-Crap4RustGate "CRAP crap4rust" @("cargo-crap4rust") -ExcludePaths @("tests/fixtures")

# ---------------------------------------------------------------------------
# Grip gate
# ---------------------------------------------------------------------------

Invoke-GripGate -Label "grip4rust self-analysis" -Threshold 50

# ---------------------------------------------------------------------------

Write-Host "`nCrap4rust and grip4rust Stage 2 passed!" -ForegroundColor Green
Pop-Location
exit 0
