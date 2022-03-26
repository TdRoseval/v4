# VS Code Config Helper (v4)

全新版本的 VS Code C++ 配置器，基于 Tauri（Rust）和 Svelte.js。

## 开发

安装 Node.js 和 Rust 环境。安装 `pnpm` 作为包管理器。

### Linux dependencies

```sh
sudo apt install libwebkit2gtk-4.0-dev libssl-dev libgtk-3-dev libappindicator3-dev librsvg2-dev
```

```sh
# 安装包依赖
pnpm i

# 开发（热更新）
pnpm tauri dev

# 编译
pnpm tauri build
```

## 比较与待办清单

| 功能             | v1  | v2  | v3  | v4  |
| ---------------- | --- | --- | --- | --- |
| MinGW 安装       | ✔️   | ✔️   | ✔️   | ✔️   |
| MinGW 检测       | ❌   | ✔️   | ✔️   | ✔️   |
| VS Code 检测     | ❌   | ✔️   | ✔️   | ✔️   |
| 扩展安装         | ❔   | ✔️   | ✔️   | ✔️   |
| 工作文件夹验证   | ✔️   | ✔️   | ✔️   | ✔️   |
| 创建测试代码     | ✔️   | ✔️   | ✔️   | ✔️   |
| C 语言支持       | ✔️   | ✔️   | ✔️   | ✔️   |
| 常用编译参数     | ❌   | ❌   | ✔️   | ✔️   |
| 外部弹窗运行     | ❌   | ✔️   | ✔️   | ✔️   |
| “新手模式”       | ❌   | ❌   | ✔️   | ✔️   |
| MinGW 编译配置   | ✔️   | ✔️   | ✔️   | ✔️   |
| LLVM MinGW       | ❌   | ❌   | ❌   | ✔️   |
| MSVC 编译配置    | ❌   | ❌   | ❌   | ✔️   |
| Linux 支持       | ❌   | ❌   | ❔   | ✔️   |
| Linux GCC        | ❌   | ❌   | ✔️   | ✔️   |
| Linux Clang      | ❌   | ❌   | ❌   | ✔️   |
| macOS 支持       | ❌   | ❌   | ❔   | ✔️   |
| Apple Clang      | ❌   | ❌   | ✔️   | ✔️   |
| 非法调试前警告   | ❌   | ❌   | ✔️   | ✔️   |
| CLI              | ❌   | ❌   | ✔️   | ✔️   |
| 管理员配置       | ✔️   | ✔️   | ❌   | ❌   |
| Windows 快捷方式 | ❌   | ❌   | ✔️   | ✔️   |
| “离线”安装扩展   | ❌   | ❌   | ✔️   | ❌   |
| 安装中文语言包   | ❌   | ❌   | ✔️   | ❌   |

| 技术信息             | v1           | v2           | v3     | v4        |
| -------------------- | ------------ | ------------ | ------ | --------- |
| 主要语言             | C#           | C#           | C++    | Rust      |
| GUI                  | Windows Form | Windows Form | Vue.js | Svelte.js |
| 可脱机运行           | ✔️            | ✔️            | ❌      | ✔️         |
| 单文件应用           | ❌            | ❌            | ✔️      | ✔️         |
| 程序体积 （Windows） | 0.4M         | 0.5M         | 4.0M   | 4.5M+     |
| <!--                 | 代码量（行） | 600+         | 1600+  | 2000+     | 2800+ | --> |
