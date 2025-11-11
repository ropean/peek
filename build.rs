fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/pk.ico");
        // Set Windows subsystem to "windows" to hide console window
        // Console will be dynamically attached if CLI args are present
        res.set_manifest(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="asInvoker" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#);
        res.compile().expect("Failed to compile Windows resources");
    }
    
    // Note: We use #![windows_subsystem = "windows"] in main.rs instead of linker flags
    // This ensures a clean GUI executable without console window on startup
}
