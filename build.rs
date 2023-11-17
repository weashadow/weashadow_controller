fn main() {
  slint_build::compile_with_config(
      "ui/main.slint",
      slint_build::CompilerConfiguration::new()
          .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
  ).unwrap();
  // println!(r"cargo:rustc-link-search=/Users/natsuki/Downloads/armv7-unknown-linux-gnueabihf/lib");
}