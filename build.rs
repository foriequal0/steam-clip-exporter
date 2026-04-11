fn main() {
    glib_build_tools::compile_resources(
        &["resources/"],
        "resources/steam-clip-exporter.gresource.xml",
        "steam-clip-exporter.gresource",
    );
}
