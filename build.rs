fn main() {
    // Rerun the build script if the app icon changes
    println!("cargo:rerun-if-changed=resources/UniversalScrobbler.icns");
}
