# Contributing to AFK-Dunld

First off, thank you for considering contributing to AFK-Dunld! It's people like you that make AFK-Dunld such a great tool.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Coding Guidelines](#coding-guidelines)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)

---

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

### Our Standards
- Be respectful and inclusive
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards other community members

---

## How Can I Contribute?

### Reporting Bugs
Before creating bug reports, please check existing issues. When you create a bug report, include as many details as possible:

- **Clear title** - Describe the issue in one sentence
- **Steps to reproduce** - Detailed steps to reproduce the behavior
- **Expected behavior** - What you expected to happen
- **Actual behavior** - What actually happened
- **Screenshots** - If applicable
- **Environment** - OS, version, etc.

### Suggesting Enhancements
Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- **Clear title** - Describe the enhancement
- **Detailed description** - Explain the feature and its benefits
- **Use cases** - Describe real-world scenarios
- **Mockups** - If applicable

### Pull Requests
- Fill in the required template
- Follow the coding guidelines
- Include tests for new features
- Update documentation
- Ensure CI passes

---

## Development Setup

### Prerequisites
```bash
# Install Node.js 18+
https://nodejs.org

# Install Rust
https://rustup.rs

# Install Tauri prerequisites
https://tauri.app/v1/guides/getting-started/prerequisites
```

### Setup
```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/afk-dunld.git
cd afk-dunld

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Project Structure
```
afk-dunld/
├── src/                    # Frontend (React/TypeScript)
│   ├── components/        # React components
│   ├── hooks/            # Custom React hooks
│   ├── stores/           # Zustand state management
│   ├── services/         # API services
│   └── utils/            # Utility functions
├── src-tauri/            # Backend (Rust)
│   ├── src/
│   │   ├── commands/     # Tauri commands
│   │   ├── core/         # Core logic
│   │   ├── database/     # Database layer
│   │   ├── network/      # Network operations
│   │   └── utils/        # Utility functions
│   └── resources/        # Bundled resources
└── docs/                 # Documentation
```

---

## Coding Guidelines

### TypeScript/React
```typescript
// Use functional components with hooks
export const MyComponent = () => {
  const [state, setState] = useState<Type>(initialValue);
  
  // Use early returns
  if (!data) return null;
  
  return <div>...</div>;
};

// Use TypeScript strictly
interface Props {
  title: string;
  onClick: () => void;
}

// Use meaningful names
const handleButtonClick = () => { /* ... */ };
```

### Rust
```rust
// Follow Rust conventions
pub struct MyStruct {
    field: String,
}

impl MyStruct {
    pub fn new(field: String) -> Self {
        Self { field }
    }
}

// Use error handling
pub async fn my_function() -> Result<String, Error> {
    // Implementation
}

// Document public APIs
/// Performs a download operation
pub async fn download(url: &str) -> Result<()> {
    // Implementation
}
```

### General Guidelines
- **DRY** - Don't Repeat Yourself
- **KISS** - Keep It Simple, Stupid
- **YAGNI** - You Aren't Gonna Need It
- Write self-documenting code
- Add comments for complex logic
- Keep functions small and focused
- Use meaningful variable names

---

## Commit Messages

### Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style changes (formatting, etc)
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `test`: Adding tests
- `chore`: Maintenance tasks

### Examples
```
feat(youtube): add playlist download support

- Implemented playlist detection
- Added batch download UI
- Updated yt-dlp integration

Closes #123
```

```
fix(download): resolve stalled download issue

Fixed issue where downloads would stall after 5 minutes
by implementing auto-recovery mechanism.

Fixes #456
```

---

## Pull Request Process

### Before Submitting
1. **Fork** the repository
2. Create a **feature branch** from `main`
3. Make your changes
4. **Test** thoroughly
5. **Update** documentation
6. **Commit** with clear messages

### Submitting
1. Push to your fork
2. Open a Pull Request
3. Fill in the PR template
4. Link related issues
5. Request review

### PR Template
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Manual testing performed
- [ ] All tests passing

## Screenshots
If applicable

## Checklist
- [ ] Code follows guidelines
- [ ] Documentation updated
- [ ] No new warnings
- [ ] Tests added
```

### Review Process
1. At least one maintainer must review
2. All CI checks must pass
3. All comments must be resolved
4. No conflicts with main branch

---

## Testing

### Frontend Tests
```bash
npm run test
```

### Backend Tests
```bash
cd src-tauri
cargo test
```

### Integration Tests
```bash
npm run test:integration
```

### Writing Tests
```typescript
// Frontend
describe('MyComponent', () => {
  it('should render correctly', () => {
    render(<MyComponent />);
    expect(screen.getByText('Title')).toBeInTheDocument();
  });
});
```

```rust
// Backend
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download() {
        let result = download("https://example.com/file").await;
        assert!(result.is_ok());
    }
}
```

---

## Documentation

### Code Documentation
- Document all public APIs
- Include examples in docstrings
- Keep docs up to date with code changes

### User Documentation
- Update README.md for user-facing changes
- Add guides to docs/ folder
- Include screenshots for UI changes

---

## Release Process

Maintainers only:

1. Update version in `package.json` and `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create release tag
4. Build binaries for all platforms
5. Create GitHub release
6. Publish release notes

---

## Getting Help

- 💬 [GitHub Discussions](https://github.com/yourusername/afk-dunld/discussions)
- 📧 Email: dev@afk-dunld.com
- 💭 Discord: [Join our server](https://discord.gg/afk-dunld)

---

## Recognition

Contributors are recognized in:
- README.md contributors section
- Release notes
- Hall of Fame (coming soon)

---

Thank you for contributing to AFK-Dunld! 🎉
