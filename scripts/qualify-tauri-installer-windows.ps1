# Spec 101 T101-07: Windows NSIS install / launch / uninstall qualification.
# CI RUNNERS ONLY (disposable). Refuses to run if MedScale app data or an
# existing MedScale install is present, so it can never touch a real user's
# data. Launches on a separate, never-shown desktop: no user input can reach
# the process, nothing is typed, and no vault is created.
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$ReleaseRoot,
    [string]$ReportPath = '',
    [int]$LaunchSeconds = 20
)
$ErrorActionPreference = 'Stop'
if (-not $IsWindows) { throw 'Windows only' }
if ($env:GITHUB_ACTIONS -ne 'true') { throw 'refusing to run outside GitHub Actions (disposable runner required)' }
$identifier = 'org.medscale.desktop.preview'
$appData = Join-Path $env:LOCALAPPDATA $identifier
$uninstallKeys = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*'
function Find-Install { Get-ItemProperty $uninstallKeys -ErrorAction SilentlyContinue | Where-Object { $_.DisplayName -eq 'MedScale' } | Select-Object -First 1 }
if (Test-Path -LiteralPath $appData) { throw "app data already present at $appData; refusing" }
if (Find-Install) { throw 'MedScale is already installed; refusing' }

$manifest = Get-Content -LiteralPath (Join-Path $ReleaseRoot 'package-manifest.json') -Raw | ConvertFrom-Json
$installer = Join-Path $ReleaseRoot $manifest.installer.path
$result = [ordered]@{ source_sha = $manifest.source_sha; installer_sha256 = $manifest.installer.sha256; INSTALL = 'FAIL'; LAUNCH = 'FAIL'; UNINSTALL = 'FAIL'; SIGNING = 'NOT_GRANTED' }

Add-Type @"
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
public static class QD {
  [StructLayout(LayoutKind.Sequential, CharSet=CharSet.Unicode)]
  public struct STARTUPINFO { public int cb; public string lpReserved; public string lpDesktop; public string lpTitle;
    public int dwX, dwY, dwXSize, dwYSize, dwXCountChars, dwYCountChars, dwFillAttribute, dwFlags; public short wShowWindow, cbReserved2;
    public IntPtr lpReserved2, hStdInput, hStdOutput, hStdError; }
  [StructLayout(LayoutKind.Sequential)] public struct PROCESS_INFORMATION { public IntPtr hProcess, hThread; public int dwProcessId, dwThreadId; }
  public delegate bool EnumProc(IntPtr h, IntPtr l);
  [DllImport("user32.dll", CharSet=CharSet.Unicode, SetLastError=true)] public static extern IntPtr CreateDesktop(string n, IntPtr d, IntPtr m, int f, uint a, IntPtr s);
  [DllImport("user32.dll")] public static extern bool CloseDesktop(IntPtr h);
  [DllImport("user32.dll")] public static extern bool EnumDesktopWindows(IntPtr desk, EnumProc cb, IntPtr l);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("kernel32.dll", CharSet=CharSet.Unicode, SetLastError=true)]
  public static extern bool CreateProcess(string app, string cmd, IntPtr pa, IntPtr ta, bool inh, uint f, IntPtr env, string cwd, ref STARTUPINFO si, out PROCESS_INFORMATION pi);
  public static List<string> Titles(IntPtr desk, uint pid) {
    var titles = new List<string>();
    EnumDesktopWindows(desk, (h, l) => { uint p; GetWindowThreadProcessId(h, out p);
      if (p == pid) { var sb = new StringBuilder(256); if (GetWindowText(h, sb, 256) > 0) titles.Add(sb.ToString()); } return true; }, IntPtr.Zero);
    return titles;
  }
}
"@

# INSTALL
$p = Start-Process -FilePath $installer -ArgumentList '/S' -Wait -PassThru
$install = Find-Install
if ($p.ExitCode -eq 0 -and $install) {
    $installDir = $install.InstallLocation.Trim('"')
    if (-not $installDir) { $installDir = Split-Path ($install.UninstallString.Trim('"')) }
    $exe = Get-ChildItem -LiteralPath $installDir -Filter '*.exe' -File | Where-Object Name -notlike 'uninstall*' | Select-Object -First 1
    if ($exe) {
        $result.INSTALL = 'PASS'
        $result.install_dir_files = @(Get-ChildItem -LiteralPath $installDir -Recurse -File | ForEach-Object { [IO.Path]::GetRelativePath($installDir, $_.FullName).Replace('\', '/') })
        $result.installed_exe = [ordered]@{ name = $exe.Name; sha256 = (Get-FileHash -LiteralPath $exe.FullName -Algorithm SHA256).Hash.ToLowerInvariant(); bytes = $exe.Length }
        $result.display_version = $install.DisplayVersion
    }
}
$result.installer_exit_code = $p.ExitCode

# LAUNCH (hidden desktop)
if ($result.INSTALL -eq 'PASS') {
    $deskName = 'medscale-101-' + [guid]::NewGuid().ToString('N').Substring(0, 8)
    $desk = [QD]::CreateDesktop($deskName, [IntPtr]::Zero, [IntPtr]::Zero, 0, 0x10000000, [IntPtr]::Zero)
    $si = New-Object QD+STARTUPINFO; $si.cb = [Runtime.InteropServices.Marshal]::SizeOf($si); $si.lpDesktop = "WinSta0\$deskName"
    $pi = New-Object QD+PROCESS_INFORMATION
    if ($desk -ne [IntPtr]::Zero -and [QD]::CreateProcess($exe.FullName, $null, [IntPtr]::Zero, [IntPtr]::Zero, $false, 0, [IntPtr]::Zero, $installDir, [ref]$si, [ref]$pi)) {
        Start-Sleep -Seconds $LaunchSeconds
        $alive = [bool](Get-Process -Id $pi.dwProcessId -ErrorAction SilentlyContinue)
        $children = @(Get-CimInstance Win32_Process | Where-Object { $_.ParentProcessId -eq $pi.dwProcessId } | Select-Object -ExpandProperty Name)
        $titles = @([QD]::Titles($desk, [uint32]$pi.dwProcessId))
        $result.launch = [ordered]@{ alive_after_seconds = $LaunchSeconds; alive = $alive; child_processes = $children; window_titles = $titles }
        if ($alive -and ($children -contains 'msedgewebview2.exe')) { $result.LAUNCH = 'PASS' }
        Get-CimInstance Win32_Process | Where-Object { $_.ProcessId -eq $pi.dwProcessId -or $_.ParentProcessId -eq $pi.dwProcessId } | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }
        Start-Sleep -Seconds 3
    } else { $result.launch = 'process could not be created on the hidden desktop' }
    if ($desk -ne [IntPtr]::Zero) { [QD]::CloseDesktop($desk) | Out-Null }
    $result.app_data_after_launch = @(if (Test-Path -LiteralPath $appData) { Get-ChildItem -LiteralPath $appData -Force | Select-Object -ExpandProperty Name })
}

# UNINSTALL
if ($install) {
    $uninstaller = $install.UninstallString.Trim('"')
    $u = Start-Process -FilePath $uninstaller -ArgumentList '/S' -Wait -PassThru
    Start-Sleep -Seconds 5  # NSIS uninstallers re-launch from a temp copy
    $stillRegistered = [bool](Find-Install)
    $residue = @(if ($installDir -and (Test-Path -LiteralPath $installDir)) { Get-ChildItem -LiteralPath $installDir -Recurse -Force | ForEach-Object { $_.FullName } })
    $result.uninstall = [ordered]@{ exit_code = $u.ExitCode; still_registered = $stillRegistered; install_dir_residue = $residue
        app_data_residue = @(if (Test-Path -LiteralPath $appData) { Get-ChildItem -LiteralPath $appData -Recurse -Force | ForEach-Object { [IO.Path]::GetRelativePath($appData, $_.FullName) } }) }
    if ($u.ExitCode -eq 0 -and -not $stillRegistered -and $residue.Count -eq 0) { $result.UNINSTALL = 'PASS' }
}

$json = $result | ConvertTo-Json -Depth 6
if ($ReportPath) { New-Item -ItemType Directory -Path (Split-Path $ReportPath) -Force | Out-Null; $json | Set-Content -LiteralPath $ReportPath -Encoding utf8 }
$json
"INSTALL=$($result.INSTALL) LAUNCH=$($result.LAUNCH) UNINSTALL=$($result.UNINSTALL) SIGNING=NOT_GRANTED"
if ($result.INSTALL -ne 'PASS' -or $result.LAUNCH -ne 'PASS' -or $result.UNINSTALL -ne 'PASS') { exit 1 }
# All three axes passed; do not let a stale native exit code (pkill, hdiutil detach, ...) fail the step.
exit 0
