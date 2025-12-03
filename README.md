# WG-Bridge

A cross-platform application for managing WireGuard VPN connections.  
The goal of this project is to provide a simple and unified way to handle WireGuard tunnels on different operating systems using Python.

---

## 🔍 Overview

This project aims to:

- Interact with WireGuard tools available on the system
- Provide an easy way to start, stop, and inspect tunnels
- Offer both a programmatic API and user interfaces (CLI and/or GUI)
- Keep the codebase maintainable, extensible, and platform-agnostic

The application is currently in an early development stage and no specific design decisions are considered final.

---

## 🚀 Getting Started (Development)

### 1. Clone the repository

```bash
git clone https://github.com/<your-username>/wg-manager
cd wg-manager
````

### 2. Create a virtual environment

```bash
python3 -m venv .venv
source .venv/bin/activate      # Linux/macOS
.venv\Scripts\activate         # Windows
```

### 3. Install dependencies

Dependencies will evolve as development progresses.

```bash
pip install -r requirements.txt
```

---

## ⚙️ Requirements

This application expects WireGuard tools to be installed on the system.
It does **not** bundle WireGuard or install system-level components.

Typical installation methods:

* **Linux** – via distribution package managers
* **macOS** – via the official WireGuard website or App Store
* **Windows** – via the official WireGuard installer

---

## 📁 Project Status

This project is under active development.
Behavior, structure, and interfaces may change as the project evolves.

---

## 🤝 Contributions

Community feedback, suggestions, and contributions are welcome.
Please open an issue or pull request to discuss proposed changes.

---

## 📜 License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.
See the [LICENSE](./LICENSE) file for more details.
