# 可暂停的未来

[![Crate](https://img.shields.io/crates/v/pausable_future.svg)](https://crates.io/crates/pausable_future)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/pausable_future)](https://github.com/xuxiaocheng0201/pausable_future/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/pausable_future)](https://github.com/xuxiaocheng0201/pausable_future/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/pausable_future)](https://github.com/xuxiaocheng0201/pausable_future/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/pausable_future)](https://github.com/xuxiaocheng0201/pausable_future/blob/master/LICENSE)

**其他语言版本：[English](README.md), [简体中文](README_zh.md).**

# 描述

一个可暂停和恢复的Future，在后台任务中非常有用。


# 用法

将以下内容添加到你的`Cargo.toml`：

```toml
[dependencies]
pausable_future = "~0.1"
```


# 示例

```rust
use std::time::Duration;

use pausable_future::Pausable;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let pausable = Pausable::new(async {
        let mut count = 0;
        loop {
            sleep(Duration::from_millis(300)).await;
            count += 1;
            println!("count: {}", count);
        }
    });
    let controller = pausable.controller();
    tokio::spawn(pausable);
    println!("spawn");
    sleep(Duration::from_secs(1)).await;
    controller.pause();
    println!("paused");
    sleep(Duration::from_secs(1)).await;
    controller.resume();
    println!("resumed");
    sleep(Duration::from_secs(1)).await;
}
```
