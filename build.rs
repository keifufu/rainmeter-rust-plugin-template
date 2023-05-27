extern crate winres;

fn main() {
  let mut res = winres::WindowsResource::new();
  // Change these
  res.set("FileVersion", "1.0.0.0"); // Plugin version
  res.set("LegalCopyright", "© 2023 - keifufu");
  // Don't change these
  res.set("ProductName", "Rainmeter");
  res.set("ProductVersion", "3.0.2.2161 (64-bit)");
  res.compile().unwrap();
}
