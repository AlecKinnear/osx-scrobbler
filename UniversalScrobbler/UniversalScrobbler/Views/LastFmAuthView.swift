import SwiftUI

struct LastFmAuthView: View {
    @Binding var config: AppConfig
    @State private var isAuthenticating = false
    @State private var authToken = ""
    @State private var errorMessage = ""
    @State private var showError = false
    @Environment(\.colorScheme) var colorScheme

    var isAuthenticated: Bool {
        !(config.lastfm?.sessionKey ?? "").isEmpty
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            HStack {
                Image("musical-note")
                    .resizable()
                    .renderingMode(.template)
                    .foregroundColor(colorScheme == .dark ? .white : .black)
                    .frame(width: 24, height: 24)

                Text("Last.fm Authentication")
                    .font(.headline)
            }

            if isAuthenticated {
                HStack {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(.green)
                    Text("Connected to Last.fm")
                        .foregroundColor(.secondary)
                    Spacer()
                    Button("Disconnect") {
                        disconnect()
                    }
                    .buttonStyle(.bordered)
                }
                .padding()
                .background(Color.green.opacity(0.1))
                .cornerRadius(8)
            } else {
                VStack(alignment: .leading, spacing: 12) {
                    Text("To authenticate with Last.fm:")
                        .font(.subheadline)
                        .foregroundColor(.secondary)

                    Text("1. Enter your API credentials below")
                        .font(.caption)
                    Text("2. Click 'Authorize' to open Last.fm")
                        .font(.caption)
                    Text("3. Grant permission on the Last.fm website")
                        .font(.caption)
                    Text("4. Return here and click 'Complete'")
                        .font(.caption)
                }
                .padding()
                .background(Color.blue.opacity(0.1))
                .cornerRadius(8)

                TextField("API Key", text: Binding(
                    get: { config.lastfm?.apiKey ?? "" },
                    set: {
                        if config.lastfm == nil {
                            config.lastfm = LastFmConfig(enabled: true, apiKey: "", apiSecret: "", sessionKey: "")
                        }
                        config.lastfm?.apiKey = $0
                    }
                ))
                .textFieldStyle(.roundedBorder)

                SecureField("API Secret", text: Binding(
                    get: { config.lastfm?.apiSecret ?? "" },
                    set: {
                        if config.lastfm == nil {
                            config.lastfm = LastFmConfig(enabled: true, apiKey: "", apiSecret: "", sessionKey: "")
                        }
                        config.lastfm?.apiSecret = $0
                    }
                ))
                .textFieldStyle(.roundedBorder)

                if !authToken.isEmpty {
                    HStack {
                        Image(systemName: "info.circle")
                            .foregroundColor(.blue)
                        Text("Authorization in progress. Click 'Complete' after granting permission.")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    .padding()
                    .background(Color.blue.opacity(0.1))
                    .cornerRadius(8)
                }

                HStack(spacing: 12) {
                    if authToken.isEmpty {
                        Button("Authorize with Last.fm") {
                            startAuth()
                        }
                        .buttonStyle(.borderedProminent)
                        .disabled(config.lastfm?.apiKey.isEmpty ?? true || config.lastfm?.apiSecret.isEmpty ?? true)
                    } else {
                        Button("Complete Authorization") {
                            Task {
                                await completeAuth()
                            }
                        }
                        .buttonStyle(.borderedProminent)
                        .disabled(isAuthenticating)

                        Button("Cancel") {
                            authToken = ""
                        }
                        .buttonStyle(.bordered)
                    }

                    if isAuthenticating {
                        ProgressView()
                            .scaleEffect(0.8)
                    }
                }
            }

            if showError {
                HStack {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .foregroundColor(.red)
                    Text(errorMessage)
                        .font(.caption)
                        .foregroundColor(.red)
                }
                .padding()
                .background(Color.red.opacity(0.1))
                .cornerRadius(8)
            }

            Link("Get Last.fm API credentials", destination: URL(string: "https://www.last.fm/api/account/create")!)
                .font(.caption)
        }
        .padding()
    }

    private func startAuth() {
        guard let apiKey = config.lastfm?.apiKey else { return }

        authToken = UUID().uuidString

        if let authURL = LastFmService.getAuthURL(apiKey: apiKey) {
            NSWorkspace.shared.open(authURL)
        }
    }

    private func completeAuth() async {
        guard let apiKey = config.lastfm?.apiKey,
              let apiSecret = config.lastfm?.apiSecret else {
            showError(message: "API credentials are missing")
            return
        }

        isAuthenticating = true
        showError = false

        do {
            let sessionKey = try await LastFmService.getSessionKey(
                apiKey: apiKey,
                apiSecret: apiSecret,
                token: authToken
            )

            config.lastfm?.sessionKey = sessionKey
            config.lastfm?.enabled = true
            authToken = ""
            ConfigManager.shared.save()

        } catch {
            showError(message: "Authentication failed. Make sure you granted permission on Last.fm.")
        }

        isAuthenticating = false
    }

    private func disconnect() {
        config.lastfm?.sessionKey = ""
        config.lastfm?.enabled = false
        ConfigManager.shared.save()
    }

    private func showError(message: String) {
        errorMessage = message
        showError = true

        DispatchQueue.main.asyncAfter(deadline: .now() + 5) {
            showError = false
        }
    }
}
