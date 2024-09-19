## UIG
An anonymous repo for the paper of UIG

## Quick Start
```shell
cd rap
git submodule update --init --recursive
./0-install-rap-rust.sh
./1-install-rap-cargo.sh
```

## Usage
Enter your Rust project folder with a `Cargo.toml` file. If your project contains a `rust-toolchain.toml` file, we recommend disabling or removing it.

### generate upg
```shell
cargo rap -upg
```
Before using this feature, make sure that `graphviz` is installed on your device.

![](https://cdn.nlark.com/yuque/0/2024/png/34484379/1726713842070-b02dcc43-9b2c-4609-8a3b-3383ae4afe1e.png)

Then, our tool will generate all upgs in folder `UPG`

![](https://cdn.nlark.com/yuque/0/2024/png/34484379/1726713925864-347b42b5-cd8f-4cef-8dd4-4500437e991f.png)

### count uig
```shell
cargo rap -uig
```

![](https://cdn.nlark.com/yuque/0/2024/png/34484379/1726713350472-f5a8290d-e356-4df0-b061-a2884ceeccc3.png)

### check doc
```shell
cargo rap -doc
```

![](https://cdn.nlark.com/yuque/0/2024/png/34484379/1726713408114-0cd2236d-59ed-4a90-bf71-0c7792022132.png)



###  check unsafe constructors
```shell
cargo rap -ucons
```

![](https://cdn.nlark.com/yuque/0/2024/png/34484379/1726713380006-46b51dd2-3f6a-4dc6-95af-a087b7570660.png)

