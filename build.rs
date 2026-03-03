fn main() {
    // This tells cargo to rerun the build script if the icon changes
    println!("cargo:rerun-if-changed=../universalescrobbler.iconset/icon_32.png");
}
