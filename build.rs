fn main() {
    glib_build_tools::compile_resources(
        &["resources/"],
        "resources/steam-clip-exporter.gresource.xml",
        "steam-clip-exporter.gresource",
    );

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        println!("cargo:rerun-if-changed=resources/icons.ico");
        let mut res = winresource::WindowsResource::new();
        res.set_icon("resources/icons.ico");
        res.compile().expect("Failed to compile Windows resource");
    }
}
