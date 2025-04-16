fn main() {
    // Generates runtime-accessible build information.
    // Collects metadata including:
    // - Git commit hash
    // - Build timestamp
    // - Compiler version
    // - Build profile
    // - etc.
    build_info_build::build_script();
}
