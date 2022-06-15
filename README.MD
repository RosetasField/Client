# | **🪄 Roseta's Field**

### You like magic stuff ? This game is for you (UwU)

<br>

# **CROSS PLATFORM GAME** (`Windows`, `Linux`)

<br>

# Install dependencies

<br>

> ## Windows

- ### <u><a href="https://win.rustup.rs/">Click to install cargo</a></u>
- ### <u><a href="https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16">Click to install VS2019 build tools</a></u>
    &emsp; `Select "Desktop development with C++`<br>
    &emsp; `Select "Universal Windows Platform build tools`
- ### <u>`Install cargo-binutils`</u>
    ```bash
    cargo install -f cargo-binutils
    ```
- ### <u>`Install llvm tools`</u>
    ```bash
    rustup component add llvm-tools-preview
    ```

<br>

> ## Linux

- ### <u>`Install cargo`</u>
    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```
- ### <u><a href="https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md">Install _**Bevy**_ dependencies</a></u>
- ### <u>`Install clang and lld`</u>
    > **Ubuntu:** ``sudo apt-get install clang lld``

    > **Arch:** ``sudo pacman -S clang lld``

<br>

# How to build ? 

```bash
cargo build --release
```

<br>

# How to run ?

```bash
cargo run --release
```

<br>

### Notes

#### This game is using the wonderful *`Bevy`* framework

<br>

# <u>_**Made with ❤️ by RqndomHax**_</u>
