# 🦀 Cat-in-a-Box (Rust + Raylib)

## ⚙️ Setup Steps

### 1. Install VSCode and WSL
Make sure you have **WSL** installed and working properly.  
Then install **Visual Studio Code** (with WSL integration).

> 💡 Tip: Open your project directly inside WSL (`code .`).

---

### 2. Install Rust on WSL
```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustc --version
```

---

### 3. Install CMake, Clang, and Dependencies

You’ll need build tools and libraries for Raylib and Rust FFI:

```bash
sudo apt update
sudo apt install build-essential git cmake clang
sudo apt install libasound2-dev libx11-dev libxrandr-dev libxi-dev \
libgl1-mesa-dev libglu1-mesa-dev libxcursor-dev libxinerama-dev \
libwayland-dev libxkbcommon-dev libssl-dev musl lldb
```

---

### 4. Add Raylib to Your Cargo.toml
Inside your Rust project, add Raylib as a dependency:

```bash
[dependencies]
raylib = "5.0"
```

---

### 5. Configure VSCode Debugging

Create (or update) your `.vscode/launch.json` file:

```bash
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug Rust Program",
            "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}",
            "args": [],
            "cwd": "${workspaceFolder}",
            "preLaunchTask": "cargo build",
            "environment": []
        }
    ]
}
```

> 🧠 **Note:** `"type": "lldb"` requires the **vadimcn.vscode-lldb** plugin.

---

### 6. Install Recommended VSCode Extensions

(((bash
code --install-extension vadimcn.vscode-lldb
code --install-extension rust-lang.rust-analyzer
code --install-extension tamasfe.even-better-toml
)))

---

## 📦 Building & Shipping

To build your project for release:

```bash
cargo build --release
```

Then copy your **asset folders** into the release directory.  
Your final structure should look something like this:

```bash
./target/release/Cat-in-a-box
./target/release/assets/
```

Run your game from the `target/release` folder and enjoy 🐱📦

---

## 🧩 Notes

- This is my **first Rust project**, so expect some… *creative* code choices.
- I left a few unused things in the code — mostly as references for future projects.
- Don’t treat this repo as a guide to "good Rust practices".  
  It’s more like a *"what happens if I do this?"* kind of playground.

---

## ❤️ Acknowledgments

- [Raylib](https://www.raylib.com/) — for making graphics fun again.  
- The Rust community — for the endless patience with newbies.  
- Me — for not giving up halfway through compilation errors.

---

> *“If it compiles, it ships.”* — Ancient Rust proverb

Ps: Written with ChatGPT because I was too lazy to do so