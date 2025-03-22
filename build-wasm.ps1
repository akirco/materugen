param(
    [string]$Target = "web",
    [string]$WasmTarget = "wasm32-unknown-unknown"
)

# Ensure wasm32 target is installed
rustup target add wasm32-unknown-unknown

# Build with wasm-pack
wasm-pack build --target web --features wasm --no-default-features

# Copy to web directory
Remove-Item -Recurse -Force web/pkg -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path web/pkg
Copy-Item -r pkg/* web/pkg/

