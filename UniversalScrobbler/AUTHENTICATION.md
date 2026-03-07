# Last.fm Authentication Guide

## Overview

The macOS Scrobbler now includes a user-friendly authentication interface for Last.fm that guides users through the OAuth-like authentication flow.

## Authentication Flow

### 1. Setup
Users need to obtain Last.fm API credentials first:
- Visit: https://www.last.fm/api/account/create
- Create an application to get API Key and API Secret

### 2. Authentication Process

The app implements Last.fm's web authentication flow:

1. **Enter Credentials**: User enters API Key and API Secret in the Settings > Services tab

2. **Authorize**: User clicks "Authorize with Last.fm" which:
   - Generates an authentication token
   - Opens Last.fm authorization page in the default browser
   - User grants permission on Last.fm website

3. **Complete**: User returns to the app and clicks "Complete Authorization" which:
   - Exchanges the token for a session key
   - Saves the session key to UserDefaults
   - Enables Last.fm scrobbling

4. **Connected**: The app now displays a "Connected to Last.fm" status with option to disconnect

## UI Features

### Authentication View (`LastFmAuthView`)
- Clean, step-by-step instructions
- Visual feedback for authentication states
- Error handling with user-friendly messages
- Status indicators (connected/disconnected)
- Direct link to API credential creation

### Settings Integration
- Integrated into Settings > Services tab
- Separate sections for Last.fm and ListenBrainz
- Rounded cards with proper spacing
- Consistent with macOS design guidelines

## Light/Dark Mode Support

All UI elements properly support both light and dark modes:
- Musical note icon renders as template (adapts to system theme)
- Color schemes use system colors
- Status messages use appropriate opacity levels
- Icons and text maintain proper contrast

## Technical Implementation

### LastFmService Extensions
Added static methods for authentication:
- `getAuthURL(apiKey:)`: Generates Last.fm authorization URL
- `getSessionKey(apiKey:apiSecret:token:)`: Exchanges token for session key

### Security
- API Secret stored in secure text field
- Session key stored in UserDefaults (consider Keychain for production)
- All API communication over HTTPS
- MD5 signature generation for API requests

## Assets

### Musical Note Icon
- Located in: `Resources/Assets.xcassets/musical-note.imageset/`
- Renders as template for automatic light/dark adaptation
- Used in:
  - Menu bar status item
  - Authentication view header
  - About page
  - App icon

### Asset Catalog Structure
```
Assets.xcassets/
├── AppIcon.appiconset/
│   ├── Contents.json
│   └── musical-note.png
├── musical-note.imageset/
│   ├── Contents.json
│   └── musical-note.png
├── AccentColor.colorset/
│   └── Contents.json
└── Contents.json
```

## Future Enhancements

Potential improvements for the authentication system:
- Store session key in Keychain instead of UserDefaults
- Add session validation on app launch
- Implement automatic session refresh
- Add ListenBrainz authentication UI (currently manual token entry)
- Show Last.fm username when connected
- Add "Test Connection" button
