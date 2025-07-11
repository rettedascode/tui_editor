# install.ps1
Write-Host "Checking for Rust..."
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Rust not found. Installing Rust..."
    Invoke-WebRequest -Uri https://static.rust-lang.org/rustup/init.exe -OutFile "$env:TEMP\rustup-init.exe"
    & "$env:TEMP\rustup-init.exe" -y
    $env:PATH += ";$env:USERPROFILE\.cargo\bin"
} else {
    Write-Host "Rust is already installed."
}

Write-Host "Building tui_editor in release mode..."
cargo build --release

# Optionally copy to a directory in PATH (e.g., user cargo bin)
$target = "$env:USERPROFILE\.cargo\bin\tui_editor.exe"
Copy-Item -Path ".\target\release\tui_editor.exe" -Destination $target -Force
Write-Host "Installed tui_editor to $target"
Write-Host "You can now run tui_editor from any PowerShell or CMD window." 