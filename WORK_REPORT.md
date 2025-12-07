## 工作说明

- 修复依赖编译错误：在 `esp-idf-svc` 中将 ALPN 与 TLS 相关的指针类型统一为 `c_char`，避免在目标平台上出现 `*const i8`/`*const u8` 不匹配。修改文件：`patches/esp-idf-svc/src/private/cstr.rs`、`patches/esp-idf-svc/src/tls.rs`，并在 `Cargo.toml` 中通过 `[patch.crates-io]` 引入本地补丁。
- 修复 `esp32-nimble` 不兼容的特性：移除已废弃的 `inline_const_pat`，将错误码匹配改成查表方式，修正多处因原始指针自动引用导致的 `dangerous_implicit_autorefs` 编译错误，更新文件：`patches/esp32-nimble/src/ble_error.rs`、`patches/esp32-nimble/src/client/ble_client.rs`、`patches/esp32-nimble/src/server/ble_characteristic.rs`、`patches/esp32-nimble/src/server/ble_descriptor.rs`，并在 `Cargo.toml` 中添加本地补丁。
- 补全缺失配置文件：根据示例拷贝 `src/wifi_config.rs.example` 为 `src/wifi_config.rs`，以解决 `cargo check` 无法找到配置文件的问题（请将其中的 Wi-Fi 与 Bot 凭据替换成真实值）。
- 结果：`cargo check` 现已通过（仅剩若干来自上游的警告，如 `static mut` 可变引用、生命周期标注建议和一个已重命名的 lint）。

如需进一步清理警告，可按照编译提示为函数添加显式生命周期、替换已重命名的 lint（`temporary_cstring_as_ptr` -> `dangling_pointers_from_temporaries`），并评估是否重构 `static mut` 的使用。
