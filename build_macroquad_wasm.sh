#!/bin/bash

# Build script for Stereo3D Stereogram Viewer with pure Macroquad WASM
set -e

echo "🚀 Building Stereo3D Stereogram Viewer with pure Macroquad WASM..."

# Check if basic-http-server is installed (for local testing)
if ! command -v basic-http-server &> /dev/null; then
    echo "📦 Installing basic-http-server for local testing..."
    cargo install basic-http-server
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf stero3d.wasm
rm -rf docs/

# Build for WASM using cargo (macroquad's recommended approach)
echo "🔨 Building for WASM..."
cargo build --target wasm32-unknown-unknown --release

# Create docs directory
echo "📁 Creating docs directory..."
mkdir -p docs

# Copy WASM file to docs directory
echo "📁 Copying WASM file..."
cp target/wasm32-unknown-unknown/release/stero3d.wasm docs/

# Copy HTML file to docs directory
echo "📁 Copying HTML file..."
cp index.html docs/

# Copy JavaScript bundle to docs directory
echo "📁 Copying JavaScript bundle..."
cp mq_js_bundle.js docs/

echo "✅ Build complete!"
echo ""
echo "🌐 To run locally:"
echo "   basic-http-server docs"
echo "   Then open http://localhost:4000"
echo ""
echo "📦 Files ready for deployment:"
echo "   - docs/stero3d.wasm"
echo "   - docs/index.html"
echo "   - docs/mq_js_bundle.js"
echo ""
echo "🚀 To deploy to GitHub Pages:"
echo "   git add docs/"
echo "   git commit -m 'Update WASM build'"
echo "   git push origin main"
