# 发布 diff_json 到 crates.io 指南

## 前置条件

1. 在 [crates.io](https://crates.io) 注册账号
2. 安装 cargo 命令行工具

## 发布步骤

### 1. 登录 crates.io

```bash
cargo login
```

输入你的 crates.io API token（可以在 https://crates.io/me 获取）

### 2. 验证项目

确保项目目录包含以下文件：
- Cargo.toml（已配置好）
- README.md（已完成）
- LICENSE-MIT 和 LICENSE-APACHE（已完成）
- CHANGELOG.md（已完成）

### 3. 检查项目

```bash
cd /Users/bytedance/RProject/diff_json

# 运行测试
cargo test

# 检查文档
cargo doc --no-deps --open

# 检查是否有警告
cargo clippy -- -D warnings

# 格式化代码
cargo fmt -- --check
```

### 4. 发布到 crates.io

```bash
# 发布到 crates.io
cargo publish
```

### 5. 验证发布

发布成功后，你可以在 https://crates.io/crates/diff_json 查看你的 crate

## 发布后的使用

发布成功后，其他项目可以通过以下方式使用：

```toml
[dependencies]
diff_json = "0.1.0"
```

## 注意事项

1. **版本号管理**：发布后，如果需要更新，必须增加版本号（遵循语义化版本）
2. **yank（撤回）**：如果发现问题，可以使用 `cargo yank` 撤回版本
   ```bash
   cargo yank --vers 0.1.0
   ```
3. **API Token 安全**：不要泄露你的 API token

## 发布前检查清单

- [ ] 所有测试通过
- [ ] 文档完整（README.md）
- [ ] 许可证文件存在
- [ ] Cargo.toml 配置正确
- [ ] 没有编译警告
- [ ] 代码格式化
- [ ] 已登录 crates.io

## 发布新版本

如果要发布新版本：

1. 更新 Cargo.toml 中的版本号
2. 更新 CHANGELOG.md
3. 运行测试
4. 发布：
   ```bash
   cargo publish
   ```

## 故障排除

### 错误：crate name already exists
如果 crate 名称已被占用，需要在 Cargo.toml 中修改名称

### 错误：invalid registry token
需要重新登录：
```bash
cargo login
```

### 错误：files are not utf-8
确保所有文件都是 UTF-8 编码

## 本地测试

在发布前，可以先在本地测试：

```bash
# 在其他项目中使用本地版本
[dependencies]
diff_json = { path = "/Users/bytedance/RProject/diff_json" }
```

这已经在 restaurant 项目中配置好了，可以运行：
```bash
cd /Users/bytedance/RProject/restaurant
cargo run --example menu_diff
```
