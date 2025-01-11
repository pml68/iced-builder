fn main() {
    println!("cargo::rerun-if-changed=fonts/icons.toml");
    iced_fontello::build("fonts/icons.toml").expect("Build icons font");
    #[cfg(windows)]
    {
        embed_resource::compile(
            "assets/windows/iced_builder.rc",
            embed_resource::NONE,
        );
        windows_exe_info::versioninfo::link_cargo_env();
    }
}
