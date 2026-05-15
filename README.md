# zsm – the lightning-fast directory jumper that learns

`zsm` is a fuzzy‑matching, self‑learning CLI tool that jumps you straight into your project directories and opens Neovim.  
Type `zsm whale` and you're instantly inside `~/Dev/whale_simulator`, ready to code.

It works like `zoxide` combined with `fzf`, but **built entirely from scratch in Rust** – with a custom Robin‑Hood hashmap under the hood.

---

## Features

- **Fuzzy matching with scoring** – smart enough to find `dotfiles-bspwm` with the query `dbm`.
- **Frecency engine** – remembers which projects you actually visit and boosts them over time.
- **Persistence** – history is saved to `~/.local/share/zsm/history.json` so it survives reboots.
- **Insanely fast** – shallow directory scan, no external dependencies at runtime.
- **Shell handshake** – a tiny shell function captures the output and does `cd` + `nvim .`.
- **Custom Robin‑Hood hashmap** – the frecency database runs on an open‑addressing hashmap you wrote from scratch, with linear probing and the Robin‑Hood collision strategy.

---

## Installation

### Prerequisites
- Rust (install via [rustup](https://rustup.rs))
- Neovim (or change the shell function to your editor)

### Build from source
```bash
git clone https://github.com/Bishesh-ops/zsm.git
cd zsm
cargo build --release```

### Set up the shell function
Add this to your ~/.zshrc or ~/.bashrc:

```bash
zsm() {
    local target_dir
    target_dir=$("$HOME/Dev/zsm/target/release/zsm" "$1")
    if [ $? -eq 0 ] && [ -n "$target_dir" ]; then
        cd "$target_dir" && nvim .
    fi
}```

 Then reload your shell:

```bash
source ~/.zshrc   # or ~/.bashrc```
## Configuration
By default, zsm scans ~/Dev (or whatever you set in the fallback).
You can override this by setting the environment variable `ZSM_BASE`:

```bash
export ZSM_BASE="$HOME/Projects"```

## Usage
```bash
zsm myproject          # jumps to the best matching project
zsm dbm                # fuzzy matches 'dotfiles-bspwm'
zsm nonexistent        # error message, no jump```

After a few uses, zsm learns. Running zsm proj twice will make it prefer the previously visited match over a new one with a similar fuzzy score.

---

## Architecture
```text
src/
  main.rs         – CLI entry point, argument validation, environment setup.
  scanner.rs      – directory traversal & fuzzy matching algorithm.
  frecent.rs      – frecency database (load/save/score) backed by custom hashmap.
  myhashmap.rs    – custom Robin‑Hood hashmap (open addressing, linear probing).```

No external runtime dependencies – just serde and dirs for persistence and platform paths.

---

## Fuzzy matching algorithm
The fuzzy scorer rewards:

* Characters that appear in order.

* Consecutive matches (big bonus).

* Word boundaries (after `-`,`_`, `.`, or **camelCase ** transitions).

It returns a numeric score; the highest‑scoring directory wins.

---

## Frecency formula
 ``score = frequency * (1 + 100 / (age_in_hours + 1))``
This gives a heavy boost to projects you’ve opened recen*ly and often.

---

## Future Roadmap
- Smart case – if the query contains uppercase, match case‑sensitively.

- Multi‑threaded scanning – speed up traversal on huge directory trees.

- TUI fallback – when multiple matches have identical scores, present a fuzzy picker (fzf integration).

- Profile‑based base directories – support scanning multiple root folders simultaneously.

- Configurable scoring weights – let users tweak fuzzy vs. frecency balance.

- Packaging – provide pre‑built binaries for Linux, macOS, and Windows.

- Benchmark suite – measure the custom hashmap performance against std::collections::HashMap.

---

## Contributing
This project is a personal learning vehicle, but if you have ideas, feel free to open an issue or pull request!
The custom hashmap (`hashmap.rs`) is deliberately built from scratch – suggestions for improving its API or performance are very welcome.

---

## License
MIT © Bishesh Shrestha (2026)

Built with ❤️ and a lot of Rust.
