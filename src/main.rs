//! CatLang Compiler CLI
//!
//! Usage: catc <INPUT>
//!
//! Arguments:
//!   <INPUT>    Input CatLang source file (e.g., abc.cat)
//!
//! Options:
//!   -h, --help       Print help
//!   -V, --version    Print version
//!   -O, --opt <LVL>  Optimization level (0-3, default: 2)
//!   --release        Enable release mode (equivalent to -O 3)

use clap::Parser;
use std::fs;
use std::path::Path;
use std::process::Command;

use catlang::compile_with_opts;

#[derive(Parser, Debug)]
#[command(name = "catc")]
#[command(author = "CatLang Team")]
#[command(version = "0.1.0")]
#[command(about = "CatLang Compiler - Compiles .cat files to .exe", long_about = None)]
struct Args {
    /// Input CatLang source file
    #[arg(required = true)]
    input: String,

    /// Optimization level (0-3)
    #[arg(short = 'O', long = "opt", default_value = "2")]
    opt_level: u8,

    /// Enable release mode (maximum optimization)
    #[arg(long = "release")]
    release: bool,
}

fn main() {
    let args = Args::parse();

    // Read input file
    let input_path = Path::new(&args.input);
    if !input_path.exists() {
        eprintln!("Error: Input file '{}' does not exist", args.input);
        std::process::exit(1);
    }

    let source = match fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", args.input, e);
            std::process::exit(1);
        }
    };

    // Get output name (replace .cat with .exe)
    let output_name = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let output_exe = format!("{}.exe", output_name);
    let zig_file = format!("{}.zig", output_name);

    // Determine optimization level
    let opt_level = if args.release {
        3
    } else {
        args.opt_level.min(3)
    };
    let zig_opt = if opt_level >= 3 {
        "ReleaseFast"
    } else if opt_level >= 2 {
        "ReleaseSafe"
    } else if opt_level >= 1 {
        "Debug"
    } else {
        "Debug"
    };

    eprintln!(
        "[catc] Compiling '{}' (optimization level: {})...",
        args.input, opt_level
    );

    // Compile CatLang to Zig
    let zig_code = match compile_with_opts(&source, opt_level) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Compilation failed: {}", e);
            std::process::exit(1);
        }
    };

    // Write Zig code to file
    match fs::write(&zig_file, &zig_code) {
        Ok(_) => eprintln!("[catc] Generated '{}'", zig_file),
        Err(e) => {
            eprintln!("Error writing Zig file: {}", e);
            std::process::exit(1);
        }
    };

    // Compile Zig to executable
    eprintln!("[catc] Building executable with Zig (-O {})...", zig_opt);
    let output_arg = format!("-femit-bin={}", output_exe);

    let mut zig_args = vec![
        "build-exe".to_string(),
        zig_file.clone(),
        output_arg,
        "-O".to_string(),
        zig_opt.to_string(),
    ];

    // Add frame pointer for Debug mode only
    if opt_level == 0 {
        zig_args.push("-fno-omit-frame-pointer".to_string());
    }

    let zig_build = Command::new("zig").args(&zig_args).output();

    match zig_build {
        Ok(result) => {
            if !result.status.success() {
                eprintln!("Zig compilation failed:");
                eprintln!("{}", String::from_utf8_lossy(&result.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!(
                "Error: Could not find 'zig' compiler. Please ensure Zig is installed and in PATH."
            );
            eprintln!("Error details: {}", e);
            // Keep zig file for debugging
            // let _ = fs::remove_file(&zig_file);
            std::process::exit(1);
        }
    }

    let _ = fs::remove_file(&zig_file);
    eprintln!("[catc] Successfully created '{}'", output_exe);
}
