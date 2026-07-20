fn main() {
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");

    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("ui/images/ico.ico"); // Optional: path to an .ico file for Windows Explorer
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resource: {e}");
        }
    }
}
