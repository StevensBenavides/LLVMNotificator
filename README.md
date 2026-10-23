# LLVM Notificator

LLVM Notificator tells you when a new LLVM version is released. It saves the release notes on your computer and shows a notification with a button to open them. It can also send the LLVM notes to Discord, and it catches up on any releases you missed while your computer was off.

## CLI

```console
$ LLVMNotificator --help
LLVMNotificator - GitHub release watcher with desktop notifications

USAGE:
    LLVMNotificator [OPTIONS]

OPTIONS:
    --check          Check for new releases now, notify, and exit
    --flush          Deliver pending notifications and exit
    --test-webhook   Validate the Discord webhook and send a test message
    --simulate       Write changelogs without notifying or updating state
    --no-notify      Generate changelogs without sending notifications
    --tag <tag>      Process a specific release (backfill)
    --config <path>  Use a custom configuration file
    --verbose        Print detailed progress
    -h, --help       Show this help
    -V, --version    Show the version
```

## Build From Source

### Requirements

- Linux with systemd user services.
- Rust 1.89 or newer and Cargo.
- A C compiler (`gcc` or `clang`) for the mimalloc allocator.
- `make`.
- A D-Bus session bus and a notification daemon that supports actions (KDE Plasma, GNOME, XFCE, etc.).
- `org.freedesktop.FileManager1` or `xdg-open`/`gio` for opening changelog folders.

### Build, install, and enable

```console
$ cd /path/to/LLVMNotificator
$ cargo build --release
$ make install
$ make enable
```

`make install` places the binary in `~/.local/bin/LLVMNotificator`, installs the systemd user unit, and creates the default configuration only when it does not exist. `make enable` enables and starts the user service.

Uninstall:

```console
$ make uninstall  # remove the binary and the systemd unit
```

## Getting Started

### Configuration file location

The default configuration file is:

```text
$XDG_CONFIG_HOME/LLVMNotificator/config.toml
```

When `XDG_CONFIG_HOME` is not set, this is:

```text
~/.config/LLVMNotificator/config.toml
```

Related paths:

| Purpose | Default path |
| --- | --- |
| Configuration | `~/.config/LLVMNotificator/config.toml` |
| GitHub token | `~/.config/LLVMNotificator/token` |
| State | `~/.local/state/LLVMNotificator/state.json` |
| Changelogs | `~/.local/share/LLVMNotificator/changelogs/vX.Y.Z/` |

The configuration file contains a Discord webhook URL when Discord delivery is enabled, so keep it private:

```console
$ chmod 600 ~/.config/LLVMNotificator/config.toml
```

### Full configuration example

```toml
repository = "llvm/llvm-project"
display_name = "LLVM"
include_prerelease = false

poll_interval_secs = 18000  # 5 hours
notification_expire_ms = 0
action_label = "Open folder"

max_response_bytes = 8388608
max_commits = 1000

[webhook]
enabled = false
url = "https://discord.com/api/webhooks/ID/TOKEN"
username = "LLVMNotificator"

[[components]]
name = "llvm"
notes_path = "llvm/docs/ReleaseNotes.md"

[[components]]
name = "clang"
notes_path = "clang/docs/ReleaseNotes.md"

[[components]]
name = "mlir"
notes_path = "mlir/docs/ReleaseNotes.md"

[[components]]
name = "flang"
notes_path = "flang/docs/ReleaseNotes.md"
```

### Configuration reference

| Key | Default | Description |
| --- | --- | --- |
| `repository` | `llvm/llvm-project` | GitHub repository in `owner/project` format. |
| `display_name` | `LLVM` | Prefix used in notifications, directory summaries, and Discord messages. |
| `include_prerelease` | `false` | Include GitHub prereleases when selecting the latest version. |
| `poll_interval_secs` | `18000` | Seconds between checks (5 hours by default). Minimum: `30`. |
| `notification_expire_ms` | `0` | Desktop notification timeout in milliseconds. `0` keeps it until dismissed. |
| `action_label` | `Open folder` | Label of the desktop notification button. |
| `max_response_bytes` | `8388608` | Maximum HTTP response body size in bytes. Minimum: `1024`. |
| `max_commits` | `1000` | Maximum number of commits stored per changelog. |
| `webhook.enabled` | `true` when `[webhook]` exists | Enable Discord webhook delivery. Set `false` to keep the URL without sending. |
| `webhook.url` | empty | Discord incoming webhook URL. Never logged. |
| `webhook.username` | empty | Optional Discord username override. |
| `components` | LLVM, Clang, MLIR, Flang | Release-note sources. Each entry has `name` and `notes_path`. |

The release tag prefix (`llvmorg-`) and the changelog folder prefix (`v`) are internal and fixed; they are not configurable.

The first component (`llvm`) is written to the top-level `ReleaseNotes.md` inside each version folder and is the only component sent to Discord. The remaining components are stored in subfolders such as `clang/ReleaseNotes.md`.

If the `[webhook]` section is present and `enabled` is omitted, Discord delivery is enabled. The example sets `enabled = false` so the placeholder URL is not used by accident; set it to `true` after adding a real webhook URL.

### Optional GitHub token

Unauthenticated GitHub API requests are limited to 60 per hour. The daemon uses conditional requests and checks every 5 hours by default, so this is normally enough. For a higher limit (5,000 requests per hour), use a GitHub personal access token with read-only access to public repositories. No write scopes are needed.

You can provide the token through the environment. This takes precedence over the token file:

```console
$ export GITHUB_TOKEN="github_pat_EXAMPLE_TOKEN"
```

Or store it in a dedicated file with mode `600`. The file must contain only the token, on a single line, without `GITHUB_TOKEN=`, quotes, or any other text:

```text
github_pat_EXAMPLE_TOKEN
```

Create the file with the correct permissions and edit it:

```console
$ install -m 600 /dev/null ~/.config/LLVMNotificator/token
$ nano ~/.config/LLVMNotificator/token
```

### Discord webhook

Create an incoming webhook in the target Discord channel, then enable it in the configuration:

```toml
[webhook]
enabled = true
url = "https://discord.com/api/webhooks/ID/TOKEN"
username = "LLVMNotificator"
```

Validate the webhook and send a short test message:

```console
$ LLVMNotificator --test-webhook
```

### Check Health

```console
$ systemctl --user status LLVMNotificator.service
$ journalctl --user -u LLVMNotificator.service -f
```

`--check` runs a single check, sends notifications, saves state, and exits. Because the process ends, the **Open folder** button is only reliable while the background service is running.

`--simulate` writes changelogs and does not update the state or send notifications. `--no-notify` also suppresses Discord delivery.

## License

MIT. Copyright (c) 2026 Stevens Benavides.
