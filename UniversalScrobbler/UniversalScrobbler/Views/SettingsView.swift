import SwiftUI

struct SettingsView: View {
    @StateObject private var configManager = ConfigManager.shared
    @State private var config: AppConfig
    @State private var validationErrors: [String] = []

    init() {
        _config = State(initialValue: ConfigManager.shared.config)
    }

    var body: some View {
        TabView {
            GeneralSettingsView(config: $config)
                .tabItem {
                    Label("General", systemImage: "gearshape")
                }

            ServicesSettingsView(config: $config)
                .tabItem {
                    Label("Services", systemImage: "network")
                }

            AppFilteringView(config: $config)
                .tabItem {
                    Label("Apps", systemImage: "app")
                }

            TextCleanupView(config: $config)
                .tabItem {
                    Label("Text Cleanup", systemImage: "textformat")
                }

            AboutView()
                .tabItem {
                    Label("About", systemImage: "info.circle")
                }
        }
        .frame(width: 600, height: 500)
        .onChange(of: config) { newValue in
            configManager.config = newValue
            configManager.save()
            validationErrors = configManager.validate()
        }
        .alert("Configuration Errors", isPresented: .constant(!validationErrors.isEmpty)) {
            Button("OK") {
                validationErrors.removeAll()
            }
        } message: {
            Text(validationErrors.joined(separator: "\n"))
        }
    }
}

struct GeneralSettingsView: View {
    @Binding var config: AppConfig

    var body: some View {
        Form {
            Section("Polling") {
                HStack {
                    Text("Refresh Interval:")
                    TextField("Seconds", value: $config.refreshInterval, format: .number)
                        .frame(width: 100)
                    Text("seconds")
                }

                HStack {
                    Text("Scrobble Threshold:")
                    TextField("Percent", value: $config.scrobbleThreshold, format: .number)
                        .frame(width: 100)
                    Text("%")
                }
                Text("Scrobble after playing this percentage of the track (50% recommended)")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .padding()
    }
}

struct ServicesSettingsView: View {
    @Binding var config: AppConfig

    var body: some View {
        Form {
            Section("Last.fm") {
                Toggle("Enable Last.fm", isOn: Binding(
                    get: { config.lastfm?.enabled ?? false },
                    set: { config.lastfm?.enabled = $0 }
                ))

                TextField("API Key", text: Binding(
                    get: { config.lastfm?.apiKey ?? "" },
                    set: { config.lastfm?.apiKey = $0 }
                ))

                SecureField("API Secret", text: Binding(
                    get: { config.lastfm?.apiSecret ?? "" },
                    set: { config.lastfm?.apiSecret = $0 }
                ))

                SecureField("Session Key", text: Binding(
                    get: { config.lastfm?.sessionKey ?? "" },
                    set: { config.lastfm?.sessionKey = $0 }
                ))
            }

            Section("ListenBrainz") {
                ForEach(config.listenbrainz.indices, id: \.self) { index in
                    VStack(alignment: .leading, spacing: 8) {
                        Toggle("Enable \(config.listenbrainz[index].name)", isOn: $config.listenbrainz[index].enabled)

                        TextField("Name", text: $config.listenbrainz[index].name)

                        SecureField("Token", text: $config.listenbrainz[index].token)

                        TextField("API URL", text: $config.listenbrainz[index].apiUrl)
                    }
                    .padding(.vertical, 4)
                }
            }
        }
        .padding()
    }
}

struct AppFilteringView: View {
    @Binding var config: AppConfig

    var body: some View {
        Form {
            Section("Filtering Options") {
                Toggle("Prompt for new apps", isOn: $config.appFiltering.promptForNewApps)
                Toggle("Scrobble from apps without bundle ID", isOn: $config.appFiltering.scrobbleUnknown)
            }

            Section("Allowed Apps") {
                List {
                    ForEach(config.appFiltering.allowedApps, id: \.self) { app in
                        Text(app)
                    }
                    .onDelete { indexSet in
                        config.appFiltering.allowedApps.remove(atOffsets: indexSet)
                    }
                }
            }

            Section("Ignored Apps") {
                List {
                    ForEach(config.appFiltering.ignoredApps, id: \.self) { app in
                        Text(app)
                    }
                    .onDelete { indexSet in
                        config.appFiltering.ignoredApps.remove(atOffsets: indexSet)
                    }
                }
            }
        }
        .padding()
    }
}

struct TextCleanupView: View {
    @Binding var config: AppConfig
    @State private var newPattern = ""

    var body: some View {
        Form {
            Section("Cleanup Options") {
                Toggle("Enable text cleanup", isOn: $config.textCleanup.enabled)
            }

            Section("Patterns") {
                List {
                    ForEach(config.textCleanup.patterns, id: \.self) { pattern in
                        Text(pattern)
                            .font(.system(.body, design: .monospaced))
                    }
                    .onDelete { indexSet in
                        config.textCleanup.patterns.remove(atOffsets: indexSet)
                    }
                }

                HStack {
                    TextField("New regex pattern", text: $newPattern)
                    Button("Add") {
                        if !newPattern.isEmpty {
                            config.textCleanup.patterns.append(newPattern)
                            newPattern = ""
                        }
                    }
                }
            }
        }
        .padding()
    }
}

struct AboutView: View {
    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "music.note")
                .font(.system(size: 64))
                .foregroundColor(.accentColor)

            Text("Universal Scrobbler")
                .font(.title)
                .bold()

            Text("Version 1.0.0")
                .font(.subheadline)
                .foregroundColor(.secondary)

            Text("A native macOS music scrobbler")
                .font(.body)

            Spacer()
        }
        .padding()
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}
