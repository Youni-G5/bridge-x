#!/bin/bash
# Run all tests for BridgeX

set -e

echo "🧪 Running BridgeX Test Suite..."
echo ""

FAILED=0

# Backend tests
echo "━━━ Backend Tests ━━━"
cd backend
if cargo test; then
    echo "✅ Backend tests passed"
else
    echo "❌ Backend tests failed"
    FAILED=1
fi

echo ""
echo "━━━ Backend Linting ━━━"
if cargo clippy -- -D warnings; then
    echo "✅ Clippy passed"
else
    echo "❌ Clippy failed"
    FAILED=1
fi

echo ""
echo "━━━ Backend Formatting ━━━"
if cargo fmt --all -- --check; then
    echo "✅ Formatting check passed"
else
    echo "❌ Formatting check failed"
    FAILED=1
fi
cd ..
echo ""

# Mobile tests
if command -v flutter &> /dev/null; then
    echo "━━━ Mobile Tests ━━━"
    cd mobile
    if flutter test; then
        echo "✅ Mobile tests passed"
    else
        echo "❌ Mobile tests failed"
        FAILED=1
    fi
    
    echo ""
    echo "━━━ Mobile Analysis ━━━"
    if flutter analyze; then
        echo "✅ Flutter analyze passed"
    else
        echo "❌ Flutter analyze failed"
        FAILED=1
    fi
    
    echo ""
    echo "━━━ Mobile Formatting ━━━"
    if dart format --output=none --set-exit-if-changed .; then
        echo "✅ Dart formatting check passed"
    else
        echo "❌ Dart formatting check failed"
        FAILED=1
    fi
    cd ..
else
    echo "⚠️  Skipping mobile tests (Flutter not installed)"
fi
echo ""

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $FAILED -eq 0 ]; then
    echo "✅ All Tests Passed!"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    exit 0
else
    echo "❌ Some Tests Failed"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    exit 1
fi
