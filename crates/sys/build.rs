//! Build script for `flipperzero-sys`
//!
//! The following environment variables are respected:
//! - `FBT_ROOT`: Location of Flipper Build Tool environment.
//! - `FBT_TOOLCHAIN_ROOT`: Location of compiler toolchain used by FBT (default: `{FBT_ROOT}/toolchain/{ARCH}-{OS}`)
//! - `UFBT_HOME`: Location of Micro Flipper Build Tool (uFBT) used if `FBT_ROOT` is not set.

use std::{
    borrow::Cow,
    env, fs,
    path::{Path, PathBuf},
    process,
    sync::LazyLock,
};

use bindgen::{EnumVariation, callbacks::ParseCallbacks};
use regex::{Captures, Regex, Replacer};
use serde_json::Value;

#[cfg(target_arch = "x86_64")]
const FBT_ARCH: &str = "x86_64";
#[cfg(target_arch = "aarch64")]
const FBT_ARCH: &str = "arm64";

#[cfg(target_os = "windows")]
const FBT_OS: &str = "windows";
#[cfg(target_os = "macos")]
const FBT_OS: &str = "darwin";
#[cfg(target_os = "linux")]
const FBT_OS: &str = "linux";

const FBT_TOOLCHAIN_PATH_ENV: &str = "FBT_TOOLCHAIN_PATH";
const UFBT_HOME_ENV: &str = "UFBT_HOME";
const TARGET: &str = "thumbv7em-none-eabihf";
const OUTFILE: &str = "bindings.rs";
const SDK_OPTS: &str = "sdk.opts";
const VISIBILITY_PUBLIC: &str = "+";
const ALLOWLIST_EXTRAS: &[&str] = &[
    "BitBuffer.*",
    "Ble.*",
    "Bt.*",
    "Button.*",
    "Byte.*",
    "Canvas.*",
    "Cli.*",
    "Composite.*",
    "Compress.*",
    "Crypto1.*",
    "DateTime.*",
    "Dialog.*",
    "DigitalSequence.*",
    "DirWalk.*",
    "Dolphin.*",
    "Elf.*",
    "EmptyScreen.*",
    "EventLoop.*",
    "Expansion.*",
    "Felica.*",
    "FileBrowser.*",
    "Flipper.*",
    "FS_.*",
    "Furi.*",
    "Gpio.*",
    "Gui.*",
    "iButton.*",
    "Icon.*",
    "Infrared.*",
    "Input.*",
    "Iso.*",
    "KeysDict.*",
    "LFRFID.*",
    "Light.*",
    "Loader.*",
    "Loading.*",
    "Locale.*",
    "Manchester.*",
    "Menu.*",
    "Mf.*",
    "mjs_.*",
    "Nfc.*",
    "Notification.*",
    "Number.*",
    "OneWire.*",
    "Path.*",
    "pb_.*",
    "Pipe.*",
    "Plugin.*",
    "Popup.*",
    "Power.*",
    "Property.*",
    "Protocol.*",
    "Rpc.*",
    "Scene.*",
    "SignalReader.*",
    "SimpleArrary.*",
    "Slix.*",
    "St25.*",
    "Storage.*",
    "Stream.*",
    "String.*",
    "Strint.*",
    "SubGhz.*",
    "Submenu.*",
    "LFRFID.*",
    "Tar.*",
    "Text.*",
    "Usb.*",
    "Validator.*",
    "VariableItem.*",
    "Varint.*",
    "Version.*",
    "View.*",
    "Widget.*",
];

#[derive(Debug, Clone)]
struct ApiSymbols {
    pub api_version: u32,
    pub headers: Vec<String>,
    pub functions: Vec<String>,
    pub variables: Vec<String>,
}

#[derive(Debug)]
struct Cb;

impl Cb {
    fn preprocess_doxygen_comments(comment: &str) -> Cow<'_, str> {
        //
        static PARAM_IN_OUT: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(\n\s*[@\\])param\[(?:\s*(in)\s*,?\s*(out)\s*|\s*(out)\s*,\s*(in)\s*)]")
                .unwrap()
        });

        struct ParamReplacer;
        impl Replacer for ParamReplacer {
            fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
                let (prefix, first, second) = (&caps[1], &caps[2], &caps[3]);
                dst.reserve(8 + prefix.len() + first.len() + second.len());

                dst.push_str(prefix);
                dst.push_str("param[");
                dst.push_str(first);
                dst.push(',');
                dst.push_str(second);
                dst.push(']');
            }
        }

        PARAM_IN_OUT.replace_all(comment, ParamReplacer)
    }
}

impl ParseCallbacks for Cb {
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(doxygen_rs::transform(&Self::preprocess_doxygen_comments(
            comment,
        )))
    }
}

/// Determine location of Micro Flipper Build Tool (uFBT).
///
/// Checks for `UFBT_HOME` environment variable and then falls back to `.ufbt` in user's home directory.
fn uftb_home() -> PathBuf {
    if let Some(path) = env::var_os(UFBT_HOME_ENV).map(PathBuf::from) {
        return path;
    }

    dirs::home_dir()
        .expect("user home directory should exist")
        .join(".ufbt")
}

/// Load symbols from `api_symbols.csv`.
fn load_symbols<T: AsRef<Path>>(path: T) -> ApiSymbols {
    let path = path.as_ref();

    let mut reader = csv::Reader::from_path(path).expect("failed to load symbol file");

    let mut api_version: u32 = 0;
    let mut headers = Vec::new();
    let mut functions = Vec::new();
    let mut variables = Vec::new();

    for record in reader.records() {
        let record = record.expect("failed to parse symbol record");
        let name = &record[0];
        let visibility = &record[1];
        let value = &record[2];

        if visibility != VISIBILITY_PUBLIC {
            continue;
        }

        match name {
            "Version" => {
                let v = value
                    .split_once('.')
                    .expect("failed to parse symbol version");
                let major: u16 = v.0.parse().unwrap();
                let minor: u16 = v.1.parse().unwrap();

                api_version = ((major as u32) << 16) | (minor as u32);
            }
            "Header" => headers.push(value.to_string()),
            "Function" => functions.push(value.to_string()),
            "Variable" => variables.push(value.to_string()),
            _ => (),
        }
    }

    ApiSymbols {
        api_version,
        headers,
        functions,
        variables,
    }
}

#[derive(Debug, Clone)]
struct SdkOpts {
    sdk_symbols: PathBuf,
    cc_args: Vec<String>,
}

/// Load `sdk.opts` file of compiler flags.
fn load_sdk_opts<T: AsRef<Path>>(sdk_root: T) -> SdkOpts {
    let sdk_root = sdk_root.as_ref();

    let path = sdk_root.join(SDK_OPTS);
    let file = fs::File::open(&path).expect("`sdk.opts` should be readable file");

    let json: Value = serde_json::from_reader(file).expect("`sdk.opts` should be valid JSON");
    let sdk_options = json
        .as_object()
        .expect("`sdk.opts` should contain JSON dict");

    // Need to use '/' on Windows, or else include paths don't work
    let sdk_root_dir = sdk_root.to_string_lossy().replace('\\', "/");

    let sdk_symbols = sdk_options
        .get("sdk_symbols")
        .and_then(Value::as_str)
        .expect("`sdk.opts` should contain `sdk_symbols` string")
        .replace("SDK_ROOT_DIR", &sdk_root_dir)
        .into();

    let cc_args = sdk_options
        .get("cc_args")
        .and_then(Value::as_str)
        .expect("`sdk.opts` should contain `cc_args` string")
        .replace("SDK_ROOT_DIR", &sdk_root_dir);
    let cc_args = shlex::split(&cc_args).expect("`cc_args` should be shlex splitable");

    SdkOpts {
        sdk_symbols,
        cc_args,
    }
}

/// Generate bindings header.
fn generate_bindings_header(api_symbols: &ApiSymbols) -> String {
    let mut lines = Vec::new();

    lines.push(format!(
        "#define API_VERSION 0x{:08X}",
        api_symbols.api_version
    ));
    lines.push("#include \"furi/furi.h\"".to_string());

    for header in &api_symbols.headers {
        lines.push(format!("#include \"{header}\""))
    }

    lines.join("\n")
}

/// Ensure that build script is running in FBT environment.
///
/// If build script is not running in FBT environment, attempt to launch one
/// from Micro Flipper Build Tool (uFBT).
fn ensure_fbt_env() {
    if env::var_os(FBT_TOOLCHAIN_PATH_ENV).is_some() {
        // Already in FBT environment
        return;
    }

    let current_exe = env::current_exe().unwrap().as_os_str().to_owned();
    let ufbt_home = uftb_home();
    eprintln!("Using UFBT_HOME: {}", ufbt_home.display());

    if !ufbt_home.is_dir() {
        panic!("`UFBT_HOME` not found - Is Micro Flipper Build Tool (uFBT) installed?");
    }

    let toolchain_path = ufbt_home.join("current");

    let child = if cfg!(windows) {
        let fbtenv = toolchain_path.join("scripts/toolchain/fbtenv.cmd");

        process::Command::new(fbtenv)
            .arg(current_exe)
            .status()
            .expect("`fbtenv.cmd` should be executable")
    } else {
        process::Command::new("/bin/sh")
            .arg("-c")
            .arg("export FBT_TOOLCHAIN_PATH=\"$1\" && . \"${FBT_TOOLCHAIN_PATH}/scripts/toolchain/fbtenv.sh\" && exec \"$2\"")
            .arg("/bin/sh")
            .arg(toolchain_path)
            .arg(current_exe)
            .status()
            .expect("`/bin/sh` should be executable")
    };

    let exit_code = child
        .code()
        .unwrap_or_else(|| if child.success() { 0 } else { 1 });

    process::exit(exit_code);
}

fn main() {
    ensure_fbt_env();

    println!("cargo::rerun-if-env-changed=FBT_TOOLCHAIN_PATH");
    println!("cargo::rerun-if-env-changed=UFBT_HOME");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let toolchain_path: PathBuf = env::var_os(FBT_TOOLCHAIN_PATH_ENV)
        .expect("`FBT_TOOLCHAIN_PATH` environment variable should be set")
        .into();

    eprintln!("Using FBT_TOOLCHAIN_PATH = {}", toolchain_path.display());

    if !toolchain_path.is_dir() {
        panic!("`FBT_TOOLCHAIN_PATH` is not a directory");
    }

    let toolchain_arch_dir = toolchain_path.join(format!("toolchain/{FBT_ARCH}-{FBT_OS}"));

    if !toolchain_arch_dir.is_dir() {
        panic!(
            "No toolchain arch directory: {}",
            toolchain_arch_dir.display()
        );
    }

    let sdk_headers = toolchain_path.join("sdk_headers");
    if !sdk_headers.is_dir() {
        panic!("No uFBT SDK headers directory: {}", sdk_headers.display());
    }

    let sdk_opts = load_sdk_opts(&sdk_headers);

    // Load SDK compiler flags
    let cc_flags: Vec<String> = sdk_opts
        .cc_args
        .into_iter()
        .map(|arg| {
            match arg.as_str() {
                // Force word relocations by disallowing MOVW / MOVT
                "-mword-relocations" => String::from("-mno-movt"),
                _ => arg,
            }
        })
        .collect();

    // Load SDK symbols
    let symbols = load_symbols(&sdk_opts.sdk_symbols);
    let bindings_header = generate_bindings_header(&symbols);

    // Toolchain include paths
    let toolchain_include = toolchain_arch_dir.join("arm-none-eabi/include");
    let toolchain_gcc_include = toolchain_arch_dir.join("lib/gcc/arm-none-eabi/12.3.1/include");
    let toolchain_gcc_include_fixed =
        toolchain_arch_dir.join("lib/gcc/arm-none-eabi/12.3.1/include-fixed");

    // Generate bindings
    eprintln!("Generating bindings for SDK {:08X}", symbols.api_version);

    let mut bindings = bindgen::builder()
        .clang_args(["-target", TARGET])
        .clang_args(["-working-directory", toolchain_path.to_str().unwrap()])
        .clang_args(["--system-header-prefix=f7_sdk/"])
        .clang_args(["-isystem", toolchain_include.to_str().unwrap()])
        .clang_args(["-isystem", toolchain_gcc_include.to_str().unwrap()])
        .clang_args(["-isystem", toolchain_gcc_include_fixed.to_str().unwrap()])
        .clang_args(&cc_flags)
        .clang_arg("-Wno-error")
        .clang_arg("-fshort-enums")
        .clang_arg("-fvisibility=default")
        .use_core()
        .parse_callbacks(Box::new(Cb))
        .default_enum_style(EnumVariation::NewType {
            is_bitfield: false,
            is_global: true,
        })
        .prepend_enum_name(false)
        .ctypes_prefix("core::ffi")
        .allowlist_var("API_VERSION")
        .rust_target(bindgen::RustTarget::nightly())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .wrap_unsafe_ops(true)
        .header_contents("header.h", &bindings_header);

    for function in &symbols.functions {
        bindings = bindings.allowlist_function(function);
    }

    for variable in &symbols.variables {
        bindings = bindings.allowlist_var(variable);
    }

    for type_ in ALLOWLIST_EXTRAS {
        bindings = bindings.allowlist_item(type_);
    }

    let bindings = match bindings.generate() {
        Ok(b) => b,
        Err(e) => {
            // Separate error output from the preceding clang diag output for legibility
            println!("\n{e}");
            panic!("failed to generate bindings")
        }
    };

    // `-working-directory` also affects `Bindings::write_to_file`
    let outfile = out_dir.join(OUTFILE);

    eprintln!("Writing bindings to {outfile:?}");
    bindings
        .write_to_file(outfile)
        .expect("`OUT_DIR` should be writable");
}
