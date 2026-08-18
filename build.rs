fn main() {
    // 1. Force the dark theme for Slint safely at compile time
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent-dark".into());

    slint_build::compile_with_config("ui/app-window.slint", config)
        .expect("Slint build failed");

    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("ui/images/ico.ico"); // Optional: path to an .ico file for Windows Explorer
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resource: {e}");
        }
    }
}