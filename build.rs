extern crate cc;
extern crate winres;

fn main() {
  let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").expect("No target architecture was set");
  if target_arch.as_str() == "x86" {
    println!("cargo:rustc-link-arg=/SAFESEH:NO");
    cc::Build::new()
      .object("api/x86/wrapper/api_wrapper.o")
      .compile("x86_api_wrapper");
  }

  let mut res = winres::WindowsResource::new();
  res.set("FileVersion", env!("CARGO_PKG_VERSION"));
  res.set("LegalCopyright", env!("CARGO_PKG_LICENSE"));
  let product_version = match target_arch.as_str() {
    "x86_64" => "3.0.2.2161 (64-bit)",
    "x86" => "3.0.2.2161 (32-bit)",
    _ => panic!("Unsupported target architecture"),
  };
  res.set("ProductVersion", product_version);
  res.set("ProductName", "Rainmeter");
  res.compile().unwrap();
}
