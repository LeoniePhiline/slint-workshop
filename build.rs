fn main() {
    slint_build::compile_with_config(
        "ui/appwindow.slint",
        slint_build::CompilerConfiguration::new().with_style(String::from("fluent-dark")),
    )
    .unwrap();
}
