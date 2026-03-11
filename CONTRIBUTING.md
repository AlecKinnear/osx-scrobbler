# Contributing to macOS Scrobbler

Thank you for your interest in contributing to macOS Scrobbler! This Swift/SwiftUI application welcomes contributions from the community.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue on GitHub with:
- A clear, descriptive title
- Steps to reproduce the problem
- Expected behavior vs. actual behavior
- Your macOS version and Xcode version
- Relevant log output (check Console.app for macOS Scrobbler logs)

### Suggesting Features

Feature suggestions are welcome! Please create an issue with:
- A clear description of the feature
- Why it would be useful
- How it might work
- Any design considerations

### Pull Requests

1. **Fork the repository** and create a new branch from `main`
2. **Make your changes** following the coding standards below
3. **Test thoroughly** on macOS 13.0+ in both light and dark modes
4. **Update documentation** (README.md, CHANGELOG.md, inline docs) as needed
5. **Commit your changes** with clear, descriptive commit messages
6. **Push to your fork** and submit a pull request

## Development Setup

### Prerequisites

- macOS 13.0 (Ventura) or later
- Xcode 14.0 or later
- Git

### Building from Source

```bash
git clone https://github.com/yourusername/swift-scrobbler.git
cd swift-scrobbler
open UniversalScrobbler.xcodeproj
```

Then build and run in Xcode (⌘R).

See [UniversalScrobbler/XCODE_SETUP.md](UniversalScrobbler/XCODE_SETUP.md) for detailed setup instructions.

### Testing

- Test all changes in both light and dark mode
- Test with different music players (Music.app, Spotify, etc.)
- Verify scrobbling to Last.fm and ListenBrainz
- Test authentication flows
- Check memory usage and performance

## Coding Standards

### Swift Style

- Follow Swift API Design Guidelines
- Use SwiftLint if available
- Keep functions focused and concise
- Use meaningful variable and function names
- Prefer `let` over `var` when possible
- Use Swift's modern concurrency features (async/await)

### SwiftUI Conventions

- Keep views small and focused
- Extract reusable components
- Use `@State`, `@Binding`, `@ObservedObject`, and `@StateObject` appropriately
- Follow Apple's Human Interface Guidelines
- Support both light and dark mode
- Use system colors and standard components when possible

### Code Quality

- Avoid force unwrapping (`!`) except where absolutely safe
- Use guard statements for early returns
- Add comments for complex logic
- Use proper error handling with do-catch
- Log important events using `NSLog`

### Commit Messages

- Use present tense ("Add feature" not "Added feature")
- First line should be under 72 characters
- Reference issues and PRs when relevant

Example:
```
Add ListenBrainz authentication UI

- Create authentication view similar to Last.fm
- Add token validation
- Update settings view layout

Fixes #42
```

## Project Structure

```
UniversalScrobbler/
├── UniversalScrobbler.xcodeproj/
└── UniversalScrobbler/
    ├── App/                          # Application entry and delegate
    │   ├── UniversalScrobblerApp.swift
    │   ├── AppDelegate.swift
    │   └── Info.plist
    ├── Models/                       # Data models
    │   ├── Track.swift
    │   ├── Config.swift
    │   └── ScrobbleService.swift
    ├── Services/                     # Business logic
    │   ├── MediaMonitor.swift
    │   ├── LastFmService.swift
    │   ├── ListenBrainzService.swift
    │   ├── MetadataEnricher.swift
    │   └── ScrobbleManager.swift
    ├── Views/                        # SwiftUI views
    │   ├── MenuBarView.swift
    │   ├── SettingsView.swift
    │   ├── LastFmAuthView.swift
    │   ├── AlbumArtWindow.swift
    │   └── AppPromptDialog.swift
    ├── Utilities/                    # Helper utilities
    │   ├── TextCleaner.swift
    │   ├── ImageCache.swift
    │   └── Extensions.swift
    └── Resources/                    # Assets and resources
        ├── Assets.xcassets/
        └── MediaRemote.h
```

## Key Areas for Contribution

### High Priority
- ListenBrainz authentication UI (similar to Last.fm)
- Keychain storage for session keys
- Launch at login support
- Additional music player support

### Nice to Have
- Statistics and charts
- Export scrobble history
- Notification center integration
- Keyboard shortcuts

### Documentation
- Improve inline documentation
- Add code examples
- Create architecture diagrams
- Write user guides

## Design Guidelines

- Follow macOS Human Interface Guidelines
- Ensure all UI works in both light and dark mode
- Use system fonts and colors
- Keep animations subtle and purposeful
- Maintain consistent spacing and alignment
- Test on different screen sizes

## Testing Checklist

Before submitting a PR, verify:
- [ ] Builds without warnings in Xcode
- [ ] Works in both light and dark mode
- [ ] No memory leaks (test with Instruments)
- [ ] Menu bar icon displays correctly
- [ ] Authentication flows work
- [ ] Scrobbling works correctly
- [ ] Settings persist correctly
- [ ] Album art displays properly

## Questions?

If you have questions about contributing:
- Open an issue for discussion
- Ask in your pull request
- Check existing documentation in the `UniversalScrobbler/` directory

Thank you for contributing to macOS Scrobbler!
