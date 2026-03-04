# Contributing to ADB Wireless Debug

Thank you for considering contributing to ADB Wireless Debug! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce** the issue
- **Expected behavior** vs actual behavior
- **Screenshots** if applicable
- **Environment details**:
  - OS version
  - ADB version (`adb version`)
  - App version
  - Device model and Android version

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- **Clear title and description**
- **Use case** - why this enhancement would be useful
- **Proposed solution** if you have one
- **Alternative solutions** you've considered

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Make your changes**:
   - Follow the existing code style
   - Add comments for complex logic
   - Update documentation if needed
3. **Test your changes**:
   - Test on multiple platforms if possible
   - Ensure existing functionality still works
4. **Commit your changes**:
   - Use clear, descriptive commit messages
   - Reference issues in commits (e.g., "Fix #123")
5. **Push to your fork** and submit a pull request

## Development Setup

1. Install prerequisites:
   - Rust (latest stable)
   - Node.js (v16+)
   - ADB

2. Clone your fork:
```bash
git clone https://github.com/yourusername/adb-wireless.git
cd adb-wireless
```

3. Install dependencies:
```bash
npm install
```

4. Run in development mode:
```bash
npm run tauri dev
```

## Code Style Guidelines

### JavaScript
- Use ES6+ features
- Use `const` and `let`, avoid `var`
- Use meaningful variable names
- Add JSDoc comments for functions
- Keep functions small and focused

### Rust
- Follow Rust standard style (use `cargo fmt`)
- Run `cargo clippy` and fix warnings
- Add documentation comments for public functions
- Handle errors properly, avoid unwrap() in production code

### CSS
- Use CSS custom properties for theming
- Follow BEM naming convention where applicable
- Keep selectors specific but not overly nested
- Comment complex layouts

## Project Structure

```
src/              - Frontend code
src-tauri/src/    - Backend Rust code
  lib.rs          - Core ADB functionality
  main.rs         - Application entry point
```

## Testing

Currently, the project relies on manual testing. When contributing:

- Test your changes with real Android devices
- Test on different OS platforms if possible
- Verify both USB and wireless device scenarios
- Check error handling with edge cases

## Building for Release

```bash
npm run tauri build
```

This creates platform-specific installers in `src-tauri/target/release/bundle/`.

## Commit Message Guidelines

- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit first line to 72 characters
- Reference issues and pull requests after the first line

Examples:
```
Add support for custom ADB port

Fix device IP detection on Android 12+
Closes #45

Update README with troubleshooting steps
```

## Questions?

Feel free to open an issue with the "question" label if you need help or clarification.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
