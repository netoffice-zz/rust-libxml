fn main() {
  if let Ok(ref s) = std::env::var("LIBXML2") {
    // println!("{:?}", std::env::vars());
    // panic!("set libxml2.");
    let p = std::path::Path::new(s);
    let fname = std::path::Path::new(
      p.file_name()
        .unwrap_or_else(|| panic!("no file name in LIBXML2 env ({s})")),
    );
    assert!(
      p.is_file(),
      "{}",
      &format!("not a file in LIBXML2 env ({s})")
    );
    println!(
      "cargo:rustc-link-lib={}",
      fname
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .strip_prefix("lib")
        .unwrap()
    );
    println!(
      "cargo:rustc-link-search={}",
      p.parent()
        .expect("no library path in LIBXML2 env")
        .to_string_lossy()
    );
  } else {
    #[cfg(any(target_family = "unix", target_os = "macos"))]
    {
      if pkg_config_dep::find() {
        return;
      }
    }

    #[cfg(windows)]
    {
      if vcpkg_dep::find() {
        return;
      }
    }

    panic!("Could not find libxml2.")
  }
}

#[cfg(any(target_family = "unix", target_os = "macos"))]
mod pkg_config_dep {
  pub fn find() -> bool {
    if pkg_config::find_library("libxml-2.0").is_ok() {
      return true;
    }
    false
  }
}

#[cfg(target_family = "windows")]
mod vcpkg_dep {
  pub fn find() -> bool {
    #[cfg(target_env = "gnu")]
    if pkg_config::find_library("libxml-2.0").is_ok() {
      return true;
    }
    #[cfg(target_env = "msvc")]
    if vcpkg::find_package("libxml2").is_ok() {
      return true;
    }
    false
  }
}
