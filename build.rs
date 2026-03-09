fn main() {
    // This tells cargo to rerun the build script if the icon changes
    println!("cargo:rerun-if-changed=resources/universalescrobbler.iconset/icon_32.png");
}
