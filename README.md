# 🚀 SmartTerm v2

A blazing-fast, daemon-based terminal context and history manager built in Rust. SmartTerm seamlessly integrates with your favorite shell (Bash, Zsh, PowerShell) to track command history, manage execution contexts, and persist session states using a sub-millisecond IPC architecture.

## ✨ Core Features

*   **Native Shell Hooks:** Zero-latency `preexec` and `precmd` hooks for Bash, Zsh, and PowerShell. No noticeable delay in your daily workflow.
*   **Daemon Architecture:** A background `smartd` service handles heavy database lifting, state synchronization, and background workers without blocking your terminal UI.
*   **Ultra-Fast IPC:** Uses Unix Domain Sockets (Linux/macOS) and local TCP (Windows) for instantaneous communication between the shell CLI and the daemon.
*   **SQLite Persistence:** WAL-mode SQLite database (`~/.smart_term_v2.sqlite`) for robust, lock-free command history and state storage.
*   **Dynamic Context Management:** Isolate your workflow with hierarchical Namespaces, Users, and Projects.
*   **Auto-Bindings:** Automatically switch contexts (e.g., active user or project) based on your current working directory.

---

## 📦 Installation

### Linux & macOS (Bash / Zsh)
Install the binaries and inject the shell hook in one command:
```bash
curl -sSL https://raw.githubusercontent.com/bbhcoder/smart_term2/main/install.sh | bash
```

### Windows (PowerShell)
Install the binaries and add the key-handlers to your PowerShell profile:
```powershell
irm https://raw.githubusercontent.com/bbhcoder/smart_term2/main/install.ps1 | iex
```

---

## 🚀 Getting Started

1.  **Start the Daemon:** 
    The daemon must be running in the background to process hooks and manage the database.
    ```bash
    smartd &
    ```
2.  **Restart your Terminal:** 
    Open a new terminal window to ensure the shell hooks (added during installation via `smart init`) are loaded.
3.  **Start Typing!**
    Every command you execute is now securely synced to your local SQLite database with its associated context.

---

## 🛠️ Command Reference

The `smart` CLI provides interactive commands to manage your current terminal context.

### 🏢 Namespace Management
Namespaces act as the highest level of isolation (e.g., separating `Personal` from `Work`).
*   `smart namespace use <name>` - Set the active namespace.
*   `smart namespace exit` - Clear the active namespace.
*   `smart namespace list` - View all previously used namespaces.

### 👤 User Management
Track commands executed under different logical roles or user profiles.
*   `smart user login <username>` - Set the active user.
*   `smart user logout` - Clear the active user.
*   `smart user list` - List available users in the system.

### 📁 Project Management
Isolate history and state for specific codebases or projects.
*   `smart project set <project_name>` - Set the active project.
*   `smart project clear` - Clear the active project.

### 🔗 Directory Bindings (Auto-Context)
SmartTerm can automatically switch your active user or project when you `cd` into specific directories (Prefetching).
*   `smart bind user <username> <path>` - Automatically switch to `<username>` when entering `<path>`.
*   `smart bind project <project_name> <path>` - Auto-activate project context in `<path>`.

*Note: Bindings are evaluated asynchronously in the `precmd` hook after every prompt render.*

---

## 🏗️ Workspace Architecture

SmartTerm v2 is designed as a modular Rust workspace:

*   **`cli` (smart):** The frontend command-line interface. It parses interactive commands and routes invisible shell hooks to the daemon via IPC.
*   **`smartd`:** The background async daemon powered by Tokio. It hosts the IPC server, manages the shared state cache, and runs periodic database sync workers.
*   **`smartcore`:** The brain of the operation. Contains the ABI protocol definitions (serialization/deserialization), the `preexec`/`precmd` execution engine, and the in-memory state cache.
*   **`db`:** Handles all SQLite interactions, schema migrations (WAL-mode initialization), and provides query abstractions for history and bindings.
*   **`shell`:** Generates native, non-intrusive shell scripts for Bash, Zsh, and PowerShell to safely hook into the terminal lifecycle without breaking existing configs.

---

## 🗄️ Database Schema

All data is stored locally in `~/.smart_term_v2.sqlite`. You can query it directly using `sqlite3`:

```bash
# View your command history
sqlite3 ~/.smart_term_v2.sqlite "SELECT command, cwd, timestamp FROM history ORDER BY timestamp DESC LIMIT 10;"

# View current session state
sqlite3 ~/.smart_term_v2.sqlite "SELECT * FROM session_state;"
```

## 📄 License
MIT License