// src/kpse_api.rs -- exposing a kpse-like API for the C/C++ code
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use libc;
use std::ffi::{CStr, OsStr};
use std::io::{stderr, Write};
use std::path::Path;
use std::ptr;
use std::os::unix::ffi::OsStrExt;

use find;

/* Emulating the kpathsea C API.  */

#[no_mangle]
pub extern fn kpse_find_file(name: *const i8, format: libc::c_int, must_exist: libc::c_int) -> *const i8 {
    /* This function can never work for Tectonic because files in the bundle
     * can't be referenced by path names. */

    let rname = unsafe { CStr::from_ptr (name) };
    let rformat = find::c_format_to_rust (format);
    let rmust_exist = must_exist != 0;
    writeln!(&mut stderr(), "WARNING: kpsezip find_file: {:?}, {:?} ({}), {}",
             rname, rformat, format, rmust_exist).expect ("stderr failed");
    ptr::null()
}

/* Our custom extensions. */

#[no_mangle]
pub extern fn kpsezip_get_readable_fd(name: *const i8, format: libc::c_int, must_exist: libc::c_int) -> libc::c_int {
    let rname = Path::new (OsStr::from_bytes (unsafe { CStr::from_ptr (name) }.to_bytes ()));
    let rformat = find::c_format_to_rust (format);
    let rmust_exist = must_exist != 0;

    let rv = match rformat {
        Some(fmt) => find::get_readable_fd (rname, fmt, rmust_exist),
        None => None
    };

    match rv {
        Some(fd) => fd,
        None => -1
    }
}