# Xcode Project Setup

After pulling these changes, you'll need to add the new files to your Xcode project:

## Files to Add

1. **LastFmAuthView.swift**
   - Location: `UniversalScrobbler/Views/LastFmAuthView.swift`
   - Target: UniversalScrobbler
   - Add to "Views" group in Xcode

2. **Assets.xcassets**
   - Location: `UniversalScrobbler/Resources/Assets.xcassets`
   - Target: UniversalScrobbler
   - Add to "Resources" group in Xcode
   - Make sure "Copy items if needed" is unchecked (it's already in the right place)

## Steps in Xcode

1. Open `UniversalScrobbler.xcodeproj` in Xcode

2. Right-click on the "Views" folder in the Project Navigator
   - Select "Add Files to UniversalScrobbler..."
   - Navigate to and select `UniversalScrobbler/Views/LastFmAuthView.swift`
   - Ensure "UniversalScrobbler" target is checked
   - Click "Add"

3. Right-click on the "Resources" folder in the Project Navigator
   - Select "Add Files to UniversalScrobbler..."
   - Navigate to and select `UniversalScrobbler/Resources/Assets.xcassets`
   - Ensure "UniversalScrobbler" target is checked
   - **Uncheck** "Copy items if needed" (file is already in correct location)
   - Click "Add"

4. Verify the Assets catalog:
   - Click on `Assets.xcassets` in Project Navigator
   - You should see:
     - AppIcon
     - AccentColor
     - musical-note
   - The musical-note image should show the music note icon

5. Build Settings Check:
   - Select the UniversalScrobbler project in Project Navigator
   - Select the UniversalScrobbler target
   - Go to "Build Settings" tab
   - Search for "Asset Catalog"
   - Verify "ASSETCATALOG_COMPILER_APPICON_NAME" is set to "AppIcon"

6. Build and Run:
   - Press Cmd+B to build
   - Fix any build errors if they appear
   - Press Cmd+R to run

## Expected Behavior

After setup, you should see:
- Musical note icon in the menu bar
- Last.fm authentication interface in Settings > Services
- "macOS Scrobbler" as the app name in About
- Icon adapts to light/dark mode automatically

## Troubleshooting

**If the icon doesn't appear in menu bar:**
- Check that Assets.xcassets is properly added to the target
- Verify the musical-note imageset has the PNG file
- Clean build folder (Cmd+Shift+K) and rebuild

**If LastFmAuthView shows errors:**
- Make sure the file is added to the UniversalScrobbler target
- Verify all imports are correct
- Check that LastFmService has the static authentication methods

**If build fails:**
- Check that all new files are in the target membership
- Verify there are no duplicate file references
- Clean derived data and rebuild
