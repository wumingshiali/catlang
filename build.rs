fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        
        // Set icon path (relative to project root)
        res.set_icon("icons/catc.ico");
        
        // Set application metadata
        res.set("ProductName", "CatLang Compiler");
        res.set("FileDescription", "CatLang to Zig Compiler");
        res.set("LegalCopyright", "Copyright (c) CatLang");
        res.set("FileVersion", "0.1.0");
        res.set("ProductVersion", "0.1.0");
        
        res.compile().expect("Failed to compile Windows resources!");
    }
}
