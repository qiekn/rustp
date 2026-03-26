# Rust Practice

> **Rustp**
> 别问我为什么这么起名字，因为之前创建了一个仓库叫 cppp → C++ Practice，那么 rust 当然也要

这个仓库用于学习 Rust，主线材料是中文的 Rust By Example：

- 中文：[https://rustwiki.org/zh-CN/rust-by-example/](https://rustwiki.org/zh-CN/rust-by-example/)
- 英文：[https://doc.rust-lang.org/rust-by-example/](https://doc.rust-lang.org/rust-by-example/)

目标不是“刷语法点”，而是以能快速上手 Rust、并最终用于 Tauri GUI 应用开发为导向来学习。

## 背景

我本身是有一丢丢经验的 C++ 开发者，所以这个仓库会带有比较强的“对照学习”风格：

- 遇到所有权、借用、生命周期、trait、模式匹配等概念时，会优先关注它们和 C++ 的异同
- 尽量用小而独立的例子来验证语言行为
- 学习方式仿照本喵之前的 C++ Practice 仓库：[qiekn/cppp](https://github.com/qiekn/cppp)

## 目录组织

当前采用 `src/bin/*.rs` 的方式组织例子，每个文件对应一个可以单独运行的小程序。

```text
.
├─ Cargo.toml
├─ README.md
└─ src
   ├─ main.rs
   └─ bin
      ├─ 1_1_0_comment.rs
      ├─ 1_2_0_format-print.rs
      ├─ 1_2_1_debug.rs
      ├─ 1_2_2_display.rs
      ├─ 1_2_3_list.rs
      └─ 1_2_4_format.rs
```

这种组织方式有几个好处：

- 每个文件都能单独编译和运行
- 不同例子互不影响
- 文件名可以直接映射到 Rust By Example 的章节顺序

## 命名约定

文件名尽量和 Rust By Example 的章节编号保持一致，例如：

- `1_1_0_comment.rs`
- `1_2_0_format-print.rs`
- `2_0_primitives.rs`

约定如下：

- 前面的数字表示章节顺序
- 后面的短横线名称表示当前主题
- 一个文件只放一个小主题，保持最小可运行

## 如何运行

### 使用 Cargo

运行某个示例：

```bash
cargo run --bin 1_2_0_format-print
```

也可以直接编译、测试、检查：

```bash
cargo build
cargo test
cargo clippy
cargo fmt
```

### 使用 Neovim

- `neovim`
- `rust_analyzer` LSP
- [qiekn/code-runner.nvim](https://github.com/qiekn/code-runner.nvim) Plugin

这个仓库的例子会保持“单文件即可运行”的形式，打开 Rust 文件后 `:Run` 即可在 neovim 内置终端执行单个 Rust 文件。

## 学习策略

刚看完 Hello World，后面的学习会尽量遵循下面的节奏：

1. 先照着 Rust By Example 完成最小示例
2. 再根据自己的理解开始胡编乱造
3. 对关键概念补充和 C++ 的对照着理解一下
4. 最后把这些基础知识迁移到实际的 Tauri 桌面程序开发中 (做点儿好玩儿的，因为不喜欢 Qt， wxWidgets 之类，碰巧又会一些 SCSS、React，了解了 Tauri 后就从 C++ 叛逃过来了)

重点会放在这些主题上： (不懂，这重点是 Codex 给我生成的，我... 姑且先小信一下它吧)

- 所有权与借用
- `struct` / `enum` / `match`
- 错误处理
- trait 与泛型
- 模块与包组织
- 常用标准库能力

## 视频记录

学习过程的视频记录在 Bilibili → [BV1p2XsBkEfe](https://www.bilibili.com/video/BV1p2XsBkEfe)。 (因为我可能会经常性遗忘，自己躺着刷刷视频就当是复习了。)

