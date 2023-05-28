extern crate winres;

fn main() {
  let mut res = winres::WindowsResource::new();
  res.set("FileVersion", env!("CARGO_PKG_VERSION"));
  res.set("LegalCopyright", env!("CARGO_PKG_LICENSE"));
  let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").expect("No target architecture was set");
  let product_version = match target_arch.as_str() {
    "x86_64" => "3.0.2.2161 (64-bit)",
    "x86" => "3.0.2.2161 (32-bit)",
    _ => panic!("Unsupported target architecture"),
  };
  res.set("ProductVersion", product_version);
  res.set("ProductName", "Rainmeter");
  res.compile().unwrap();
}
