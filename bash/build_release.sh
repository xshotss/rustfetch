#!/bin/bash
set -e  # Exit immediately on any error

# READ BEFORE CONTINUING



# this script is not intended for use by the users
# if you want to download the latest stable release, just go to download/












echo "🔧 Rustfetch Release Build Script"
echo "================================="

# Validate git state
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ Error: Git working directory is not clean. Commit or stash changes first."
    git status --short
    exit 1
fi

# Check we're on main branch (adjust if your main branch is named differently)
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ] && [ "$CURRENT_BRANCH" != "master" ]; then
    echo "❌ Error: Not on main/master branch. Current branch: $CURRENT_BRANCH"
    exit 1
fi

echo "📦 Running cargo fmt..."
cargo fmt --check || {
    echo "❌ Code formatting issues. Please run 'cargo fmt' manually and review changes."
    exit 1
}

echo "🔍 Running clippy..."
cargo clippy -- -D warnings || {
    echo "❌ Clippy found warnings or errors. Please fix manually."
    exit 1
}

echo "🧪 Running tests..."
cargo test --verbose || {
    echo "❌ Tests failed. Aborting release build."
    exit 1
}

echo "🏗️ Building release binary..."
cargo build --release || {
    echo "❌ Build failed."
    exit 1
}

echo "📋 Creating release directory..."
mkdir -p download

echo "📦 Copying binary to download/"
cp target/release/rustfetch download/ || {
    echo "❌ Failed to copy binary."
    exit 1
}

echo "✅ Build successful! Binary size:"
du -h download/rustfetch

echo "🔍 Final verification:"
file download/rustfetch
./download/rustfetch version

echo "🎉 Release build completed successfully!"
echo "💡 Remember to:"
echo "   - Create a proper git tag: git tag v0.1.0"
echo "   - Push the tag: git push origin v0.1.0"
echo "   - Update CHANGELOG.md"
echo "   - Create GitHub release with release notes"