import SwiftUI
import AppKit

enum AppChoice {
    case allow
    case ignore
}

func showAppPromptDialog(bundleId: String) -> AppChoice {
    let alert = NSAlert()
    alert.messageText = "New App Detected"
    alert.informativeText = "The app \"\(bundleId)\" is playing media. Would you like to scrobble from this app?"
    alert.alertStyle = .informational

    alert.addButton(withTitle: "Allow")
    alert.addButton(withTitle: "Ignore")

    let response = alert.runModal()

    return response == .alertFirstButtonReturn ? .allow : .ignore
}
