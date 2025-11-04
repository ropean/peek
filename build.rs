fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("src/app.ico");
        res.compile().expect("Failed to compile Windows resources");
    }
}
