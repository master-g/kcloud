#!/usr/bin/env bash
#
# Setup pre-commit hooks for kloud project
#

set -e

echo "Setting up pre-commit hooks for kloud..."

# Check if pre-commit config exists
if [ ! -f ".pre-commit-config.yaml" ]; then
    echo "Error: .pre-commit-config.yaml not found"
    exit 1
fi

# Check if pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
    echo "Installing pre-commit..."
    pip install pre-commit
fi

# Install the pre-commit hooks
echo "Installing pre-commit hooks..."
pre-commit install

# Run pre-commit on all files to verify setup
echo "Verifying setup..."
pre-commit run --all-files || {
    echo "Warning: Some pre-commit checks failed. This is normal on first run."
    echo "You can fix issues manually or run 'pre-commit run --all-files' later."
}

echo "Pre-commit hooks installed successfully!"
echo ""
echo "Available commands:"
echo "  pre-commit run              - Run hooks on staged files"
echo "  pre-commit run --all-files  - Run hooks on all files"
echo "  pre-commit uninstall       - Uninstall hooks"
```

```/dev/null/sh#L1-66
#!/usr/bin/env bash
#
# Setup pre-commit hooks for kloud project
#

set -e

echo "Setting up pre-commit hooks for kloud..."

# Check if pre-commit config exists
if [ ! -f ".pre-commit-config.yaml" ]; then
    echo "Error: .pre-commit-config.yaml not found"
    exit 1
fi

# Check if pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
    echo "Installing pre-commit..."
    pip install pre-commit
fi

# Install the pre-commit hooks
echo "Installing pre-commit hooks..."
pre-commit install

# Run pre-commit on all files to verify setup
echo "Verifying setup..."
pre-commit run --all-files || {
    echo "Warning: Some pre-commit checks failed. This is normal on first run."
    echo "You can fix issues manually or run 'pre-commit run --all-files' later."
}

echo "Pre-commit hooks installed successfully!"
echo ""
echo "Available commands:"
echo "  pre-commit run              - Run hooks on staged files"
echo "  pre-commit run --all-files  - Run hooks on all files"
echo "  pre-commit uninstall       - Uninstall hooks"
