# ProtonFix Manager

ProtonFix Manager is a lightweight, high-performance desktop application for Linux designed to help you manage Steam's Proton compatibility prefixes. It provides a simple graphical interface to perform common tasks that would otherwise require command-line knowledge.

## Features

- **Discover Steam Games:** Automatically scans your system to find your Steam installation and installed games.
- **Run Executables:** Easily run an arbitrary `.exe` file (like a mod installer or a non-Steam game) inside a specific game's Proton prefix, ensuring compatibility.
- **Prefix Management (Planned):** Future versions will include features for advanced prefix management, such as launching `winecfg`, `winetricks`, or browsing the prefix's virtual C: drive.

## Tech Stack

- **Language:** [Rust](https://www.rust-lang.org/)
- **GUI Framework:** [GPUI](https://github.com/zed-industries/gpui) (The high-performance, GPU-accelerated UI toolkit from the creators of the Zed editor).

## Building from Source

1.  **Install Dependencies:** Ensure you have the necessary system libraries for GUI development.
    *   **Fedora/RHEL:** `sudo dnf install -y libxcb-devel libxkbcommon-devel libxkbcommon-x11-devel wayland-devel pkg-config`
    *   **Debian/Ubuntu:** `sudo apt-get install -y libxcb1-dev libxkbcommon-dev libxkbcommon-x11-dev`
    *   **Arch Linux:** `sudo pacman -S --noconfirm libxcb libxkbcommon libxkbcommon-x11`

2.  **Build the Application:**
    ```sh
    cargo build --release
    ```

3.  **Run the Application:**
    ```sh
    cargo run
    ```