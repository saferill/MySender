fn main() {
    // VERSIONINFO metadata for the Windows exe, mirroring app/windows/runner/Runner.rc.
    // SignPath requires ProductName/ProductVersion on all signed binaries.
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        let mut res = winresource::WindowsResource::new();
        res.set("ProductName", "MySender");
        res.set("FileDescription", "MySender CLI");
        res.set("CompanyName", "MySender");
        res.set("OriginalFilename", "localsend-cli.exe");
        res.set("InternalName", "localsend-cli");
        res.set("LegalCopyright", "Copyright (C) 2026 MySender Project");
        // FileVersion/ProductVersion are derived from CARGO_PKG_VERSION automatically.
        res.compile().expect("failed to compile Windows resources");
    }
}