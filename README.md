# **NUCLEO-F767ZI-RS**

使用 rust 建立的 nucleo-f767zi 個人實驗性質專案

## **開發前準備**

準備好 nucleo-f767zi

安裝 [stm32cubeprogrammer](https://www.st.com/en/development-tools/stm32cubeprog.html)

安裝 [rust](https://doc.rust-lang.org/cargo/getting-started/installation.html) 的各種工具

推薦用 VS Code 開發

並根據 [.vscode/extensions.json](.vscode/extensions.json) 安裝推薦的延伸模組

在安裝好 probe-rs 的延伸模組的同時

若電腦未安裝 probe-rs

VS Code 右下角會有通知出現安裝 probe-rs

順勢點擊即可下載

## **開發**

建議先用 stm32cubeprogrammer 確認目前 nucleo-f767zi 的 OB nDBANK

若沒有打勾 (0) 代表 dual-bank

打勾 (1) 代表 single-bank

然後查看 [cargo.toml](cargo.toml) dependencies 的部分

確認 embassy-stm32 的 features 是否有對應的 dual-bank 或 single-bank

再使用以下命令進行建置

```bash
# debug
cargo build

# release
cargo build --release
```

第一次建置可能會出現找不到 thumbv7em-none-eabihf

並會提示我們使用以下命令安裝

```bash
rustup target add thumbv7em-none-eabihf 
```

安裝好後即可建置成功

並可以使用以下命令查看建置後的靜態 ram 與 flash 消耗大小

```bash
# 需要先使用以下命令安裝 cargo size 副命令
#cargo install cargo-binutils
#rustup component add llvm-tools-preview

# debug
cargo size
# release
cargo size --release
```

## **燒錄並運行**

使用以下命令進行燒錄並運行

```bash
# debug
cargo run

# release
cargo run --release
```

## **除錯**

透過 VS Code 執行並偵錯

已經設定好 [.vscode/launch.json](.vscode/launch.json) 和 [.vscode/tasks.json](.vscode/tasks.json)

點擊開始偵錯按鈕或 F5 即可建置並除錯

## **量產燒錄**

建置完後會看到 [target\thumbv7em-none-eabihf\release\nucleo-f767zi](target\thumbv7em-none-eabihf\release\nucleo-f767zi)

複製副本並加上副檔名 .elf

即可使用 stm32cubeprogrammer 進行燒錄
