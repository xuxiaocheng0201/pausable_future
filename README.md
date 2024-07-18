# Pausable Future

[![Crate](https://img.shields.io/crates/v/pausable_future.svg)](https://crates.io/crates/pausable_future)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/pausable_future)](https://github.com/xuxiaocheng0201/pausable_future/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/pausable_future)](https://github.com/xuxiaocheng0201/pausable_future/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/pausable_future)](https://github.com/xuxiaocheng0201/pausable_future/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/pausable_future)](https://github.com/xuxiaocheng0201/pausable_future/blob/master/LICENSE)

**Read this in other languages: [English](README.md), [简体中文](README_zh.md).**

# Description

Pausable and resumable future/stream, useful in background tasks.


# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pausable_future = "~0.2"
```


# Example

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
