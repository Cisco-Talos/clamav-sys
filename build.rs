// Copyright (C) 2020-2023 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//
// Authors: Jonas Zaddach, Scott Hutton
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
// MA 02110-1301, USA.

use std::env;
use std::path::PathBuf;

// Generate bindings for these functions:
const BINDGEN_FUNCTIONS: &[&str] = &[
    "cl_cleanup_crypto",
    "cl_cvdfree",
    "cl_cvdparse",
    "cl_debug",
    "cl_engine_addref",
    "cl_engine_compile",
    "cl_engine_free",
    "cl_engine_get_num",
    "cl_engine_get_str",
    "cl_engine_new",
    "cl_engine_set_clcb_engine_compile_progress",
    "cl_engine_set_clcb_engine_free_progress",
    "cl_engine_set_clcb_file_inspection",
    "cl_engine_set_clcb_file_props",
    "cl_engine_set_clcb_hash",
    "cl_engine_set_clcb_meta",
    "cl_engine_set_clcb_post_scan",
    "cl_engine_set_clcb_pre_cache",
    "cl_engine_set_clcb_pre_scan",
    "cl_engine_set_clcb_sigload",
    "cl_engine_set_clcb_sigload_progress",
    "cl_engine_set_clcb_stats_add_sample",
    "cl_engine_set_clcb_stats_decrement_count",
    "cl_engine_set_clcb_stats_flush",
    "cl_engine_set_clcb_stats_get_hostid",
    "cl_engine_set_clcb_stats_get_num",
    "cl_engine_set_clcb_stats_get_size",
    "cl_engine_set_clcb_stats_remove_sample",
    "cl_engine_set_clcb_stats_submit",
    "cl_engine_set_clcb_virus_found",
    "cl_engine_set_num",
    "cl_engine_set_stats_set_cbdata",
    "cl_engine_set_str",
    "cl_engine_settings_apply",
    "cl_engine_settings_copy",
    "cl_engine_settings_free",
    "cl_engine_stats_enable",
    "cl_fmap_close",
    "cl_fmap_open_handle",
    "cl_fmap_open_memory",
    "cl_init",
    "cl_initialize_crypto",
    "cl_load",
    "cl_retdbdir",
    "cl_retflevel",
    "cl_retver",
    "cl_scandesc",
    "cl_scandesc_callback",
    "cl_scanfile",
    "cl_scanfile_callback",
    "cl_scanmap_callback",
    "cl_set_clcb_msg",
    "cl_strerror",
    "cli_append_virus",
    "cli_ctx",
    "cli_dbgmsg_no_inline",
    "cli_errmsg",
    "cli_get_debug_flag",
    "cli_getdsig",
    "cli_infomsg_simple",
    "cli_versig2",
    "cli_warnmsg",
    "lsig_increment_subsig_match",
    "cl_cvdunpack_ex",
    "cl_cvdverify_ex",
    "cl_scandesc_ex",
    "cl_scanmap_ex",
    "cl_scanfile_ex",
    "cl_fmap_set_name",
    "cl_fmap_get_name",
    "cl_fmap_set_path",
    "cl_fmap_get_path",
    "cl_fmap_get_fd",
    "cl_fmap_get_size",
    "cl_fmap_set_hash",
    "cl_fmap_have_hash",
    "cl_fmap_will_need_hash_later",
    "cl_fmap_get_hash",
    "cl_fmap_get_data",
    "cl_scan_layer_get_fmap",
    "cl_scan_layer_get_parent_layer",
    "cl_scan_layer_get_type",
    "cl_scan_layer_get_recursion_level",
    "cl_scan_layer_get_object_id",
    "cl_scan_layer_get_last_alert",
    "cl_scan_layer_get_attributes",
    "cl_engine_set_scan_callback",
];

// Generate bindings for these types (structs, prototypes, etc.):
const BINDGEN_TYPES: &[&str] = &[
    "cl_cvd",
    "clcb_file_props",
    "clcb_meta",
    "clcb_post_scan",
    "clcb_pre_scan",
    "cli_ac_data",
    "cli_ac_result",
    "cli_matcher",
    "time_t",
];

// Generate "newtype" enums for these C enums
const BINDGEN_ENUMS: &[&str] = &["cl_engine_field", "cl_error_t", "cl_msg"];

const BINDGEN_CONSTANTS: &[&str] = &[
    "CL_DB_.*",
    "CL_INIT_DEFAULT",
    "CL_SCAN_.*",
    "ENGINE_OPTIONS_.*",
    "LAYER_ATTRIBUTES_.*",
];

fn bindgen_with_include_paths(
    mut bindings: bindgen::Builder,
    include_paths: &[PathBuf],
) -> bindgen::Builder {
    for include_path in include_paths {
        bindings = bindings
            .clang_arg("-I")
            .clang_arg(include_path.to_string_lossy().as_ref());
    }

    bindings
}

fn env_paths(name: &str) -> Option<Vec<PathBuf>> {
    let value = env::var_os(name)?;
    let paths = env::split_paths(&value).collect::<Vec<_>>();
    Some(paths)
}

fn validate_manual_clamav_configuration() -> bool {
    let has_library = env::var_os("CLAMAV_LIBRARY").is_some();
    let has_include = env::var_os("CLAMAV_INCLUDE").is_some();

    match (has_library, has_include) {
        (false, false) => false,
        (true, true) => true,
        (true, false) => {
            panic!("CLAMAV_INCLUDE must also be set when CLAMAV_LIBRARY is provided")
        }
        (false, true) => {
            panic!("CLAMAV_LIBRARY must also be set when CLAMAV_INCLUDE is provided")
        }
    }
}

#[cfg(windows)]
fn clamav_link_kind() -> &'static str {
    match env::var("CLAMAV_STATIC").as_deref() {
        Ok("1") => "static",
        Ok(_) | Err(env::VarError::NotPresent) => "dylib",
        Err(env::VarError::NotUnicode(_)) => {
            panic!("CLAMAV_STATIC must be valid Unicode when set")
        }
    }
}

#[cfg(windows)]
fn clamav_link_name(library_path: &std::path::Path) -> String {
    let file_name = library_path
        .file_name()
        .and_then(|name| name.to_str())
        .expect("CLAMAV_LIBRARY must point to a valid library filename");

    file_name
        .strip_suffix(".lib")
        .expect("CLAMAV_LIBRARY must point to a .lib file on Windows")
        .to_string()
}

#[cfg(windows)]
fn configure_manual_clamav_library() {
    let library_path =
        PathBuf::from(env::var_os("CLAMAV_LIBRARY").expect(
            "CLAMAV_LIBRARY environment variable must be set when using manual ClamAV paths",
        ));
    let library_dir = library_path
        .parent()
        .expect("CLAMAV_LIBRARY must include a parent directory");
    let library_kind = clamav_link_kind();
    let library_name = clamav_link_name(&library_path);

    println!(
        "cargo:rustc-link-search=native={}",
        library_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib={library_kind}={library_name}");
}

#[cfg(not(windows))]
fn configure_manual_clamav_library() {
    let library_path =
        PathBuf::from(env::var_os("CLAMAV_LIBRARY").expect(
            "CLAMAV_LIBRARY environment variable must be set when using manual ClamAV paths",
        ));
    let library_dir = library_path
        .parent()
        .expect("CLAMAV_LIBRARY must include a parent directory");

    println!(
        "cargo:rustc-link-search=native={}",
        library_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=dylib=clamav");
}

fn probe_from_env() -> Option<Vec<PathBuf>> {
    configure_manual_clamav_library();
    let include_paths = env_paths("CLAMAV_INCLUDE")
        .expect("CLAMAV_INCLUDE environment variable must be set when using manual ClamAV paths");

    Some(include_paths)
}

fn generate_bindings(customize_bindings: &dyn Fn(bindgen::Builder) -> bindgen::Builder) {
    let mut bindings = bindgen::Builder::default();
    for function in BINDGEN_FUNCTIONS {
        bindings = bindings.allowlist_function(function);
    }

    for typename in BINDGEN_TYPES {
        bindings = bindings.allowlist_type(typename);
    }

    for typename in BINDGEN_ENUMS {
        bindings = bindings.newtype_enum(typename);
    }

    for constant in BINDGEN_CONSTANTS {
        bindings = bindings.allowlist_var(constant);
    }

    bindings = bindings
        .header("wrapper.h")
        // C doc comments can contain prose that rustdoc interprets as doctest code.
        // Suppress generated comments so `cargo test` does not fail on invalid doctests.
        .generate_comments(false)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    bindings = customize_bindings(bindings);

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn cargo_common() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=CLAMAV_LIBRARY");
    println!("cargo:rerun-if-env-changed=CLAMAV_INCLUDE");
    println!("cargo:rerun-if-env-changed=CLAMAV_STATIC");
    println!("cargo:rerun-if-env-changed=OPENSSL_INCLUDE");
}

#[cfg(unix)]
fn probe_system_include_paths() -> Vec<PathBuf> {
    pkg_config::Config::new()
        .atleast_version("1.5")
        .probe("libclamav")
        .unwrap()
        .include_paths
}

#[cfg(windows)]
fn probe_system_include_paths() -> Vec<PathBuf> {
    vcpkg::find_package("clamav")
        .unwrap_or_else(|err| panic!("Failed to locate ClamAV with vcpkg: {err}"))
        .include_paths
}

#[cfg(not(any(unix, windows)))]
fn probe_system_include_paths() -> Vec<PathBuf> {
    panic!("Unsupported platform")
}

fn main() {
    let use_manual_clamav = validate_manual_clamav_configuration();
    let mut include_paths = if use_manual_clamav {
        probe_from_env().expect("manual ClamAV configuration should provide include paths")
    } else {
        probe_system_include_paths()
    };

    if let Some(openssl_include_paths) = env_paths("OPENSSL_INCLUDE") {
        include_paths.extend(openssl_include_paths);
    }

    cargo_common();

    generate_bindings(&|x: bindgen::Builder| -> bindgen::Builder {
        bindgen_with_include_paths(x, &include_paths)
    });
}
