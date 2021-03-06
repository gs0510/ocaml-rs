#[allow(unused)]
use std::io::{BufRead, Write};

#[cfg(feature = "link")]
const CC_LIB_PREFIX: &str = "NATIVECCLIBS=";

#[cfg(feature = "link")]
fn cc_libs(ocaml_path: &str) -> std::io::Result<Vec<String>> {
    let path = format!("{}/Makefile.config", ocaml_path);
    let f = std::io::BufReader::new(std::fs::File::open(path)?);

    for line in f.lines() {
        if let Ok(line) = line {
            if line.starts_with(CC_LIB_PREFIX) {
                let line: Vec<_> = line.split("=").collect();
                let line = line[1].split(" ");
                return Ok(line
                    .filter_map(|x| {
                        if x == "" {
                            None
                        } else {
                            Some(x.replace("-l", ""))
                        }
                    })
                    .collect());
            }
        }
    }
    Ok(vec![])
}

#[allow(unused)]
fn link(out_dir: std::path::PathBuf, ocamlopt: String, ocaml_path: &str) -> std::io::Result<()> {
    let mut f = std::fs::File::create(out_dir.join("runtime.ml")).unwrap();
    write!(f, "")?;

    assert!(std::process::Command::new(&ocamlopt)
        .args(&["-output-complete-obj", "-o"])
        .arg(out_dir.join("rt.o"))
        .arg(out_dir.join("runtime.ml"))
        .status()?
        .success());

    let ar = std::env::var("AR").unwrap_or_else(|_| "ar".to_string());
    assert!(std::process::Command::new(&ar)
        .arg("rcs")
        .arg(out_dir.join("libruntime.a"))
        .arg(out_dir.join("rt.o"))
        .status()?
        .success());

    #[cfg(feature = "link")]
    for lib in cc_libs(ocaml_path)? {
        println!("cargo:rustc-link-lib={}", lib);
    }

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=runtime");

    println!("cargo:rustc-link-search={}", ocaml_path);

    println!("cargo:rustc-link-lib=static=asmrun");

    Ok(())
}

#[allow(unused)]
fn run() -> std::io::Result<()> {
    let ocamlopt = std::env::var("OCAMLOPT").unwrap_or_else(|_| "ocamlopt".to_string());
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let version = std::process::Command::new(&ocamlopt)
        .arg("-version")
        .output()?;

    let version = std::str::from_utf8(&version.stdout).unwrap().trim();

    let ocaml_path = std::process::Command::new(&ocamlopt)
        .arg("-where")
        .output()?;

    let ocaml_path = std::str::from_utf8(&ocaml_path.stdout).unwrap().trim();

    let bin_path = format!("{}/../../bin/ocamlopt", ocaml_path);

    let mut f = std::fs::File::create(out_dir.join("ocaml_compiler")).unwrap();
    std::io::Write::write_all(&mut f, bin_path.as_bytes()).unwrap();

    // Write OCaml version to file
    let mut f = std::fs::File::create(out_dir.join("ocaml_version")).unwrap();
    std::io::Write::write_all(&mut f, version.as_bytes()).unwrap();

    // Write OCaml path to file
    let mut f = std::fs::File::create(out_dir.join("ocaml_path")).unwrap();
    std::io::Write::write_all(&mut f, ocaml_path.as_bytes()).unwrap();

    let split: Vec<&str> = version.split('.').collect();

    let major = split[0].parse::<usize>().unwrap();
    let minor = split[1].parse::<usize>().unwrap();

    if major >= 4 && minor >= 10 {
        // This feature determines whether or not caml_local_roots should
        // use the caml_state struct or the caml_local_roots global
        println!("cargo:rustc-cfg=caml_state");
    }

    #[cfg(feature = "link")]
    link(out_dir, bin_path, ocaml_path)?;

    Ok(())
}

fn main() {
    #[cfg(not(feature = "docs-rs"))]
    let _ = run();
}
