import SwiftUI
import AppKit

class AlbumArtWindowController: NSWindowController {
    convenience init(imageUrl: String) {
        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 400, height: 400),
            styleMask: [.titled, .closable, .resizable],
            backing: .buffered,
            defer: false
        )

        window.title = "Album Art"
        window.center()
        window.contentView = NSHostingView(rootView: AlbumArtView(imageUrl: imageUrl))

        self.init(window: window)
    }
}

struct AlbumArtView: View {
    let imageUrl: String
    @State private var image: NSImage?
    @State private var isLoading = true

    var body: some View {
        ZStack {
            if let image = image {
                Image(nsImage: image)
                    .resizable()
                    .aspectRatio(contentMode: .fit)
            } else if isLoading {
                ProgressView()
                    .scaleEffect(1.5)
            } else {
                VStack {
                    Image(systemName: "photo")
                        .font(.system(size: 64))
                        .foregroundColor(.gray)
                    Text("Failed to load album art")
                        .foregroundColor(.secondary)
                }
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Color(NSColor.windowBackgroundColor))
        .task {
            image = await ImageCache.shared.loadImage(from: imageUrl)
            isLoading = false
        }
    }
}

func showAlbumArtWindow(imageUrl: String) {
    let windowController = AlbumArtWindowController(imageUrl: imageUrl)
    windowController.showWindow(nil)
    windowController.window?.makeKeyAndOrderFront(nil)
}
