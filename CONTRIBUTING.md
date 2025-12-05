# Contributing to BridgeX

Thank you for considering contributing to BridgeX! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Commit Conventions](#commit-conventions)
- [Pull Request Process](#pull-request-process)
- [Testing Guidelines](#testing-guidelines)
- [Code Style](#code-style)

## Code of Conduct

This project adheres to a Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior by opening an issue.

## Getting Started

### Prerequisites

1. **Install dependencies**:
   ```bash
   ./scripts/bootstrap.sh
   ```

2. **Fork and clone**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/bridge-x.git
   cd bridge-x
   ```

3. **Create a branch**:
   ```bash
   git checkout -b feat/your-feature-name
   ```

## Development Workflow

### 1. Pick an Issue

- Check [open issues](https://github.com/Youni-G5/bridge-x/issues)
- Comment to claim an issue
- For new features, open an issue first to discuss

### 2. Make Changes

- Write code following our [Code Style](#code-style)
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass locally

### 3. Test Your Changes

```bash
# Run all tests
./scripts/run_all_tests.sh

# Or test individual components
cd backend && cargo test
cd mobile && flutter test
```

### 4. Commit Your Changes

Follow [Conventional Commits](#commit-conventions):

```bash
git add .
git commit -m "feat: add clipboard sync for images"
```

### 5. Push and Create PR

```bash
git push origin feat/your-feature-name
```

Then open a Pull Request on GitHub.

## Commit Conventions

We use [Conventional Commits](https://www.conventionalcommits.org/):

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, no logic change)
- **refactor**: Code refactoring
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Maintenance tasks (dependencies, CI, etc.)
- **ci**: CI/CD changes

### Examples

```bash
feat(backend): add WebRTC signaling server
fix(mobile): resolve QR scanner crash on Android 13
docs(readme): update installation instructions
ci(rust): add clippy linting to workflow
```

### Scope (optional)

- `backend` - Rust backend
- `desktop` - Tauri desktop app
- `mobile` - Flutter mobile app
- `docs` - Documentation
- `ci` - CI/CD workflows

## Pull Request Process

### PR Checklist

Before submitting, ensure:

- [ ] Code follows project style guidelines
- [ ] All tests pass (`./scripts/run_all_tests.sh`)
- [ ] New code has tests
- [ ] Documentation updated (README, API docs, etc.)
- [ ] Commit messages follow conventions
- [ ] No new secrets or credentials committed
- [ ] CI workflows pass
- [ ] Branch is up-to-date with `main`

### PR Template

When opening a PR, provide:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested this

## Screenshots (if applicable)

## Related Issues
Closes #123
```

### Review Process

1. Maintainers will review within 2-3 days
2. Address requested changes
3. Once approved, PR will be merged
4. Delete your feature branch after merge

## Testing Guidelines

### Backend (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        // Test implementation
    }

    #[tokio::test]
    async fn test_api_endpoint() {
        // Async test
    }
}
```

### Mobile (Flutter)

```dart
void main() {
  testWidgets('Widget test', (WidgetTester tester) async {
    // Test implementation
  });

  test('Unit test', () {
    // Test implementation
  });
}
```

### Integration Tests

- Place in `backend/tests/` or `mobile/integration_test/`
- Test real-world scenarios
- Mock external dependencies

## Code Style

### Rust

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Maximum line length: 100 characters

### Dart/Flutter

- Run `flutter format .` before committing
- Run `flutter analyze` and fix issues
- Follow [Effective Dart](https://dart.dev/guides/language/effective-dart)
- Use `const` constructors where possible

### General

- Write clear, self-documenting code
- Add comments for complex logic
- Use meaningful variable names
- Keep functions small and focused
- Avoid premature optimization

## Documentation

### Code Documentation

**Rust:**
```rust
/// Encrypts data using AES-GCM.
///
/// # Arguments
/// * `data` - The plaintext data to encrypt
/// * `key` - The encryption key
///
/// # Returns
/// Encrypted data or error
pub fn encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    // Implementation
}
```

**Dart:**
```dart
/// Scans QR code and extracts pairing information.
///
/// Returns [PairingInfo] if successful, null otherwise.
Future<PairingInfo?> scanQRCode() async {
  // Implementation
}
```

### README Updates

- Update if you add features
- Keep examples working
- Update architecture diagrams if needed

## Security

### Never Commit:

- Private keys or secrets
- API tokens
- Passwords or credentials
- `.env` files (use `.env.example` instead)

### Security Best Practices:

- Use GitHub Secrets for CI/CD credentials
- Review dependencies for vulnerabilities
- Follow principle of least privilege
- Report security issues privately (see [SECURITY.md](SECURITY.md))

## Questions?

- Open a [Discussion](https://github.com/Youni-G5/bridge-x/discussions)
- Check existing issues
- Ask in your PR comments

## Recognition

Contributors will be recognized in:
- Release notes
- Contributors section
- Special mentions for significant contributions

Thank you for contributing to BridgeX! 🚀
