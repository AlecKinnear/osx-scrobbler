import SwiftUI

struct MenuBarView: View {
    @ObservedObject var scrobbleManager: ScrobbleManager
    let onLove: () -> Void
    let onShowAlbumArt: () -> Void
    let onSettings: () -> Void
    let onQuit: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            if let nowPlaying = scrobbleManager.nowPlayingTrack {
                Button(action: onShowAlbumArt) {
                    VStack(alignment: .leading, spacing: 4) {
                        HStack {
                            Image(systemName: "play.circle.fill")
                                .foregroundColor(.green)
                            Text("Now Playing:")
                                .font(.caption)
                                .foregroundColor(.secondary)
                        }
                        Text(nowPlaying)
                            .font(.system(.body, design: .rounded))
                            .lineLimit(2)
                    }
                    .padding(.vertical, 8)
                    .padding(.horizontal, 12)
                    .frame(maxWidth: 300, alignment: .leading)
                }
                .buttonStyle(PlainButtonStyle())

                if scrobbleManager.albumArtUrl != nil {
                    AlbumArtThumbnailView(url: scrobbleManager.albumArtUrl)
                        .frame(height: 120)
                }

                Divider()
            }

            if let lastScrobbled = scrobbleManager.lastScrobbledTrack {
                VStack(alignment: .leading, spacing: 4) {
                    HStack {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundColor(.blue)
                        Text("Last Scrobbled:")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    Text(lastScrobbled)
                        .font(.system(.body, design: .rounded))
                        .lineLimit(2)
                }
                .padding(.vertical, 8)
                .padding(.horizontal, 12)

                Divider()
            }

            Button(action: onLove) {
                HStack {
                    Image(systemName: scrobbleManager.isLoved ? "heart.fill" : "heart")
                        .foregroundColor(.red)
                    Text("Love Track")
                }
            }
            .disabled(scrobbleManager.nowPlayingTrack == nil)

            Button(action: onSettings) {
                HStack {
                    Image(systemName: "gearshape")
                    Text("Settings...")
                }
            }

            Divider()

            Button(action: onQuit) {
                HStack {
                    Image(systemName: "power")
                    Text("Quit")
                }
            }
        }
        .frame(width: 300)
    }
}

struct AlbumArtThumbnailView: View {
    let url: String?

    @State private var image: NSImage?

    var body: some View {
        Group {
            if let image = image {
                Image(nsImage: image)
                    .resizable()
                    .aspectRatio(contentMode: .fit)
            } else {
                Rectangle()
                    .fill(Color.gray.opacity(0.2))
                    .overlay(
                        ProgressView()
                    )
            }
        }
        .task {
            guard let url = url else { return }
            image = await ImageCache.shared.loadImage(from: url)
        }
    }
}
