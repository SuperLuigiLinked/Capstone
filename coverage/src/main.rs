/*
   Code Coverage generator script.
*/

/// Whether or not User-Acceptance Tests should be run.
const RUN_UA_TESTS: bool = cfg!(feature = "UA_TESTS");

/// Whether or not Wyn Tests should be run.
const RUN_WYN: bool = cfg!(feature = "WYN") || !cfg!(feature = "RGE");

/// Whether or not RGE Tests should be run.
const RUN_RGE: bool = cfg!(feature = "RGE") || !cfg!(feature = "WYN");

/// Files that should be ignored in Coverage Results.
/// * `.*registry` to ignore external dependencies.
/// * `.*gui-sys` to ignore gui-sys crate, as it merely re-exports dependencies.
/// * `.*tests` to ignore tests, because we only want coverage for the libraries themselves.
/// * `.*log.rs` to ignore debug logging/string functions.
/// * `.*events.rs` to ignore Event Handler, as the default callbacks are all intentionally empty.
const IGNORE_FILENAME_REGEX: &str = if RUN_WYN && RUN_RGE {
    "-ignore-filename-regex=(.*registry)|(.*gui-sys)|(.*tests)|(.*log.rs)|(.*events.rs)"
} else if !RUN_WYN {
    "-ignore-filename-regex=(.*registry)|(.*gui-sys)|(.*tests)|(.*log.rs)|(.*events.rs)|(wyn)"
} else if !RUN_RGE {
    "-ignore-filename-regex=(.*registry)|(.*gui-sys)|(.*tests)|(.*log.rs)|(.*events.rs)|(rge)"
} else {
    unreachable!()
};

// ================================================================================================================================ //

use std::{
    ffi::OsString,
    fs::ReadDir,
    path::PathBuf,
    process::{Command, Stdio},
};

// ================================================================================================================================ //

pub fn main() {
    // Step 1
    install_tools();

    // Step 2
    clean_files();

    // Step 3
    run_tests(RUN_UA_TESTS);

    // Step 4
    collect_coverage();

    // Step 5
    report_coverage();

    // Step 6
    export_report();
    export_coverage();
    export_lcov();

    println!("{:=<64}\n", "");
}

// ================================================================================================================================ //

#[allow(unused)]
fn dbg_cmd(cmd: &Command) {
    let mut buf = OsString::new();

    let prog = cmd.get_program();
    buf.push(prog);

    for arg in cmd.get_args() {
        buf.push(" ");
        buf.push("\"");
        buf.push(arg);
        buf.push("\"");
    }

    println!("Running Command:\n`{}`", buf.to_string_lossy());
}

// ================================================================================================================================ //

#[allow(unused)]
fn current_path() -> PathBuf {
    let path = std::env::current_dir().expect("Unable to get Current Directory.");
    assert!(path.is_dir(), "Current Directory does not exist!");
    path
}

#[allow(unused)]
fn wyn_path() -> PathBuf {
    let mut path = current_path();
    path.push("wyn");
    assert!(path.is_dir(), "Wyn Directory does not exist!");
    path
}

#[allow(unused)]
fn rge_path() -> PathBuf {
    let mut path = current_path();
    path.push("rge");
    assert!(path.is_dir(), "RGE Directory does not exist!");
    path
}

#[allow(unused)]
fn cov_path() -> PathBuf {
    let mut path = current_path();
    path.push("coverage");
    assert!(path.is_dir(), "Coverage Directory does not exist!");
    path
}

#[allow(unused)]
fn output_path() -> PathBuf {
    let mut path = current_path();
    path.push("coverage");
    path.push("data");

    std::fs::create_dir_all(&path).expect("Unable to create Output Directory!");

    path
}

#[allow(unused)]
fn deps_path() -> PathBuf {
    let mut path = current_path();
    path.push("target");
    path.push("debug");
    path.push("deps");
    assert!(path.is_dir(), "Deps Directory does not exist!");
    path
}

#[allow(unused)]
fn profdata_path() -> PathBuf {
    let mut path = output_path();
    path.push("coverage.profdata");
    path
}

#[allow(unused)]
fn lcov_path() -> PathBuf {
    let mut path = output_path();
    path.push("lcov.info");
    path
}

#[allow(unused)]
fn covreport_path() -> PathBuf {
    let mut path = output_path();
    path.push("coverage-report.txt");
    path
}

#[allow(unused)]
fn covdetails_path() -> PathBuf {
    let mut path = output_path();
    path.push("coverage-details.html");
    path
}

// -------------------------------------------------------------------------------------------------------------------------------- //

#[allow(unused)]
fn wyn_files() -> ReadDir {
    wyn_path()
        .read_dir()
        .expect("Unable to read Wyn Directory files.")
}

#[allow(unused)]
fn rge_files() -> ReadDir {
    rge_path()
        .read_dir()
        .expect("Unable to read RGE Directory files.")
}

#[allow(unused)]
fn cov_files() -> ReadDir {
    cov_path()
        .read_dir()
        .expect("Unable to read Coverage Directory files.")
}

#[allow(unused)]
fn output_files() -> ReadDir {
    output_path()
        .read_dir()
        .expect("Unable to read Output Directory files.")
}

#[allow(unused)]
fn deps_files() -> ReadDir {
    deps_path()
        .read_dir()
        .expect("Unable to read Deps Directory files.")
}

#[allow(unused)]
fn src_files() -> Vec<PathBuf> {
    let mut files = Vec::new();

    if RUN_WYN {
        src_rec(&mut files, wyn_files());
    }

    if RUN_RGE {
        src_rec(&mut files, rge_files());
    }

    files
}

fn src_rec(files: &mut Vec<PathBuf>, dir: ReadDir) {
    for entry in dir.flatten() {
        if let Ok(ftype) = entry.file_type() {
            if ftype.is_dir() {
                let path = entry.path();

                let is_tests = path.ends_with("tests");
                let is_examples = path.ends_with("examples");

                if !(is_tests || is_examples) {
                    let dir = path.read_dir().expect("Unable to read sub-directory.");
                    src_rec(files, dir);
                }
            } else if ftype.is_file() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        files.push(path);
                    }
                }
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

fn profraw_files() -> Vec<PathBuf> {
    output_files()
        .filter_map(|res| res.ok())
        .filter_map(|entry| {
            let path = entry.path();
            let osname = path.file_name().expect("Unable to read File Name!");
            let fname = osname.to_string_lossy();
            let ftype = entry.file_type().expect("Unable to get File Type!");

            let is_file = ftype.is_file();
            let is_profraw = fname.ends_with(".profraw");

            (is_file && is_profraw).then_some(path)
        })
        .collect()
}

fn exe_files() -> Vec<PathBuf> {
    deps_files()
        .filter_map(|res| res.ok())
        .filter_map(|entry| {
            let path = entry.path();
            let osname = path.file_name().expect("Unable to read File Name!");
            let fname = osname.to_string_lossy();
            let ftype = entry.file_type().expect("Unable to get File Type!");

            let is_file = ftype.is_file();
            let is_exe = fname.ends_with(".exe");
            let isnt_coverage = !fname.starts_with("coverage");

            (is_file && is_exe && isnt_coverage).then_some(path)
        })
        .collect()
}

// ================================================================================================================================ //

fn install_tools() {
    println!("{:=^64}\n", " Installing Dependencies ");

    // rustup component add llvm-tools-preview
    {
        println!("{:-<64}\n", "---- LLVM-Tools ");

        let status = Command::new("rustup")
            .args(["component", "add", "llvm-tools-preview"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("Could not run `rustup component add llvm-tools-preview`.");

        println!();
        if status.success() {
            println!("* Success: [{status}]");
        } else {
            println!("* Failure: [{status}]");
            std::process::exit(1);
        }
        println!()
    }

    // cargo install cargo-binutils
    {
        println!("{:-<64}\n", "---- Cargo-Binutils ");

        let status = Command::new("cargo")
            .args(["install", "cargo-binutils"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("Could not run `cargo install cargo-binutils`.");

        println!();
        if status.success() {
            println!("* Success: [{status}]");
        } else {
            println!("* Failure: [{status}]");
            std::process::exit(1);
        }
        println!();
    }

    // cargo install rustfilt
    {
        println!("{:-<64}\n", "---- Rustfilt ");

        let status = Command::new("cargo")
            .args(["install", "rustfilt"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("Could not run `cargo install rustfilt`.");

        println!();
        if status.success() {
            println!("* Success: [{status}]");
        } else {
            println!("* Failure: [{status}]");
            std::process::exit(1);
        }
        println!();
    }
}

// ================================================================================================================================ //

fn clean_files() {
    println!("{:=^64}\n", " Cleaning `.profraw` files ");

    let files = profraw_files();
    let mut deleted = 0;
    let mut failed = 0;

    println!("* Found {} files.", files.len());

    for path in files {
        match std::fs::remove_file(path) {
            Ok(_) => deleted += 1,
            Err(_) => failed += 1,
        }
    }

    println!("* {deleted} files deleted.");
    if failed > 0 {
        println!("* {failed} files failed to delete.");
        std::process::exit(1);
    }
    println!();
}

// ================================================================================================================================ //

fn run_tests(include_ua_tests: bool) {
    println!("{:=^64}\n", " Running Tests ");

    if RUN_WYN {
        test_lib("wyn", false);
        if include_ua_tests {
            test_lib("wyn", true);
        }
    }

    if RUN_RGE {
        test_lib("rge", false);
        if include_ua_tests {
            test_lib("rge", true);
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------------------- //

fn test_lib(lib: &str, include_ua_tests: bool) {
    if include_ua_tests {
        println!(
            "{:-<64}\n",
            format!("---- Testing {lib} User-Acceptance Tests ")
        );
    } else {
        println!("{:-<64}\n", format!("---- Testing {lib} "));
    }

    let outfile = {
        let mut path = output_path();
        let fname = format!("{lib}-%p-%m.profraw");
        path.push(fname);
        path
    };

    let status = {
        let mut cmd = Command::new("cargo");
        cmd.arg("test");
        cmd.arg("--no-fail-fast");
        cmd.args(["-p", lib]);

        if include_ua_tests {
            cmd.args(["--", "--ignored", "--nocapture"]);
        }

        cmd.env("RUSTFLAGS", "-C instrument-coverage")
            .env("LLVM_PROFILE_FILE", outfile)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("Could not run `cargo test`.")
    };

    println!();
    if status.success() {
        println!("* Success: [{status}]");
    } else {
        println!("* Failure: [{status}]");
        std::process::exit(1);
    }
    println!();
}

// ================================================================================================================================ //

fn collect_coverage() {
    println!("{:=^64}\n", " Collecting Coverage Data ");

    let infiles = profraw_files();
    let outfile = profdata_path();

    let status = Command::new("rust-profdata")
        .arg("merge")
        .arg("-sparse")
        .args(infiles)
        .args(["-o", &outfile.to_string_lossy()])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Could not run `llvm-profdata merge`.");

    println!();
    if status.success() {
        println!("* Success: [{status}]");
    } else {
        println!("* Failure: [{status}]");
        std::process::exit(1);
    }
    println!();
}

// -------------------------------------------------------------------------------------------------------------------------------- //

fn report_coverage() {
    println!("{:=^64}\n", " Generating Coverage Report ");

    let profile = profdata_path();
    let binfiles = exe_files();

    let mut cmd = Command::new("rust-cov");
    cmd.arg("report");

    cmd.arg(IGNORE_FILENAME_REGEX);

    cmd.args(["-instr-profile", &profile.to_string_lossy()]);

    for filepath in binfiles {
        cmd.args(["-object", &filepath.to_string_lossy()]);
    }

    let output = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not run `llvm-cov report`.");

    let status = output.status;

    println!();
    if status.success() {
        println!("* Success: [{status}]");
    } else {
        println!("* Failure: [{status}]");
        std::process::exit(1);
    }
    println!();

    std::fs::write(covreport_path(), output.stdout).expect("Unable to export Coverage Report.");
}

// -------------------------------------------------------------------------------------------------------------------------------- //

fn export_report() {
    println!("{:=^64}\n", " Exporting Coverage Report ");

    let profile = profdata_path();
    let binfiles = exe_files();

    let mut cmd = Command::new("rust-cov");
    cmd.arg("report");

    cmd.arg(IGNORE_FILENAME_REGEX);

    cmd.args(["-instr-profile", &profile.to_string_lossy()]);

    for filepath in binfiles {
        cmd.args(["-object", &filepath.to_string_lossy()]);
    }

    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not run `llvm-cov report`.");

    let status = output.status;

    println!();
    if status.success() {
        println!("* Success: [{status}]");
    } else {
        println!("* Failure: [{status}]");
        std::process::exit(1);
    }
    println!();

    std::fs::write(covreport_path(), output.stdout).expect("Unable to export Coverage Report.");
}

// -------------------------------------------------------------------------------------------------------------------------------- //

fn export_coverage() {
    println!("{:=^64}\n", " Exporting Coverage Details ");

    let profile = profdata_path();
    let binfiles = exe_files();

    let mut cmd = Command::new("rust-cov");
    cmd.arg("show");

    cmd.arg("-format=html");
    cmd.arg("-Xdemangler=rustfilt");
    cmd.arg(IGNORE_FILENAME_REGEX);

    cmd.args(["-instr-profile", &profile.to_string_lossy()]);

    for filepath in binfiles {
        cmd.args(["-object", &filepath.to_string_lossy()]);
    }

    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not run `llvm-cov show`.");

    let status = output.status;

    println!();
    if status.success() {
        println!("* Success: [{status}]");
    } else {
        println!("* Failure: [{status}]");
        std::process::exit(1);
    }
    println!();

    std::fs::write(covdetails_path(), output.stdout).expect("Unable to export Coverage Report.");
}

// -------------------------------------------------------------------------------------------------------------------------------- //

fn export_lcov() {
    println!("{:=^64}\n", " Exporting Coverage Info ");

    let output = {
        let profile = profdata_path();
        let infiles = exe_files();

        let mut cmd = Command::new("rust-cov");
        cmd.arg("export");

        cmd.arg("-format=lcov");

        cmd.arg(IGNORE_FILENAME_REGEX);

        cmd.args(["-instr-profile", &profile.to_string_lossy()]);

        for filepath in infiles {
            cmd.args(["-object", &filepath.to_string_lossy()]);
        }

        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .output()
            .expect("Could not run `llvm-cov report`.");

        let status = output.status;

        println!();
        if status.success() {
            println!("* Success: [{status}]");
        } else {
            println!("* Failure: [{status}]");
            std::process::exit(1);
        }
        println!();

        output.stdout
    };

    {
        let outfile = lcov_path();

        print!("Writing to file...");
        let res = std::fs::write(outfile, output);
        match res {
            Ok(_) => println!(" success!"),
            Err(_) => println!(" failure!"),
        }
        res.expect("Unable to export coverage data to file!");
        println!();
    }
}

// ================================================================================================================================ //
