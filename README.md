# 🚀 SmartTerm v2

A blazing-fast, daemon-based terminal context and history manager built in Rust. SmartTerm seamlessly integrates with your shell (Bash, Zsh, PowerShell) to track command history, manage execution contexts, and persist session states using a sub-millisecond IPC architecture.

## ✨ Features
- **Native Shell Hooks:** Zero-latency `preexec` and `precmd` hooks for Bash, Zsh, and PowerShell.
- **Daemon Architecture:** Background `smartd` service handles heavy database lifting without blocking your terminal UI.
- **Ultra-Fast IPC:** Uses Unix Domain Sockets (Linux/macOS) and local TCP (Windows) for instant communication.
- **SQLite Persistence:** WAL-mode SQLite database for robust command history and state storage.
- **Context Management:** Isolate your workflow with hierarchical namespaces.

## 📦 Installation

### Linux & macOS (Bash/Zsh)
```bash
curl -sSL https://raw.githubusercontent.com/bbhcoder/smart_term2/main/install.sh | bash
```

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/bbhcoder/smart_term2/main/install.ps1 | iex
```

## 🚀 Quick Start

1. Start the daemon in the background (if not automatically configured):
   ```bash
   smartd &
   ```
2. Manage your terminal context dynamically:
   ```bash
   smart namespace use Backend
   smart namespace exit
   ```
3. All executed commands and context switches are now securely synced to `~/.smart_term_v2.sqlite`.

## 🏗️ Workspace Architecture
- `cli`: Command-line interface and hook router (`smart`).
- `smartd`: Async IPC server and background sync daemon.
- `smartcore`: In-memory state cache and ABI protocol definition.
- `db`: SQLite models, schema, and query abstractions.
- `shell`: Native shell hook generators and injectors.