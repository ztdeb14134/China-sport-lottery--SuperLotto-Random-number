# myLottoNumber

myLottoNumber 是一个用 Rust 编写的中国体育彩票“超级大乐透”选号与模拟工具，支持多种投注方式和自动选号，并带有进度条和概率测试功能。

## 功能简介

- **支持多种投注类型**：单式、复式、胆拖（KeyFiller）等。
- **自动选号**：根据不同玩法自动生成随机号码。
- **进度条显示**：选号过程带有彩色进度条，体验更佳。
- **概率测试**：可模拟大量选号，验证特定号码出现的概率。
- **命令行交互**：可通过命令行参数切换测试模式或正常选号。

## 依赖

- [rand](https://crates.io/crates/rand) 随机数生成
- [chrono](https://crates.io/crates/chrono) 日期时间处理
- [colored](https://crates.io/crates/colored) 彩色输出
- [crossterm](https://crates.io/crates/crossterm) 终端控制

## 使用方法

### 1. 普通选号

直接运行：

```sh
cargo run
```

根据当天星期自动选择不同的投注策略，并输出推荐号码。

### 2. 概率/性能测试

运行：

```sh
cargo run -- test
```

或

```sh
cargo run -- --test
```

将进入概率测试模式，模拟大量选号中指定数字组的次数并实时显示进度和结果。

### 3. 单元测试

```sh
cargo test -- --nocapture
```

可查看测试用例和中间输出。

## 主要代码结构

- `src/main.rs`：程序入口，根据日期选择玩法并输出结果。
- `src/China_sports_lottery.rs`：核心逻辑，包括选号、进度条、概率测试等。

## 示例输出

```
第1注: 03 12 19 25 34 : 05 06
第2注: 07 09 13 19 27 : 04 08
Enter to quit...
```

## 进阶用法

- 支持自定义倍数、追加投注等参数（详见代码）。
- 可扩展更多玩法和自定义策略。

---

如有建议或问题，欢迎提 issue
