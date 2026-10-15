//! Compiles libarchive's RAR and RAR5 readers (vendor/libarchive, libarchive 3.8.9, BSD-2-Clause; release tarball
//! libarchive-3.8.9.tar.xz, SHA-256 888c934f9d95648ecb9163dc8e23ab80a476ecb81a8f1154704a227b5b676dde) into a static
//! library. Only the read core, the two RAR readers and what they call are built: no compression libraries, no
//! iconv, no crypto. The configuration in vendor/config is written for MSVC; other targets build without RAR.

use std::path::Path;

const SOURCES: &[&str] = &[
    "archive_acl.c",
    "archive_blake2s_ref.c",
    "archive_blake2sp_ref.c",
    "archive_check_magic.c",
    "archive_entry.c",
    "archive_entry_sparse.c",
    "archive_entry_xattr.c",
    "archive_ppmd7.c",
    "archive_read.c",
    "archive_read_support_format_rar.c",
    "archive_read_support_format_rar5.c",
    "archive_string.c",
    "archive_string_sprintf.c",
    "archive_time.c",
    "archive_util.c",
    "archive_virtual.c",
    "archive_windows.c",
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=vendor");
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() != Ok("msvc") {
        return;
    }
    let src = Path::new("vendor/libarchive");
    let mut build = cc::Build::new();
    build
        .include("vendor/config")
        .include(src)
        .define("HAVE_CONFIG_H", None)
        .define("LIBARCHIVE_STATIC", None)
        .define("_CRT_SECURE_NO_WARNINGS", None)
        // Each global in its own section, so the linker drops the unused ones like unused functions.
        .flag("/Gw")
        .warnings(false);
    for file in SOURCES {
        build.file(src.join(file));
    }
    build.compile("deadloader_libarchive_rar");
    // archive_util.c makes temp-file names with BCryptGenRandom; the RAR readers never call it.
    println!("cargo:rustc-link-lib=bcrypt");
}
