# process-end-slack-notifier (`pesn`)

A CLI tool that monitors processes by PID and sends a Slack notification when they terminate.

## Requirements

- Linux (depends on the `/proc` filesystem)
- Slack Incoming Webhook URL

## Installation

```bash
cargo install --path .
```

## Usage

### Basic workflow

1. Register a Slack notification destination
2. Register the PID of the process you want to monitor
3. Run `pesn check` periodically (e.g. via cron)

### 1. Managing destinations

#### Add a destination

```bash
pesn destination add --name <name> --url <webhook-url> [--memo <memo>]
```

| Option  | Short | Required | Description |
|---------|-------|----------|-------------|
| `--name` | `-n` | Yes | Unique identifier for this destination |
| `--url`  | `-u` | Yes | Slack Incoming Webhook URL |
| `--memo` | `-m` | No  | Optional note |

**Example:**

```bash
pesn destination add -n my-channel -u https://hooks.slack.com/services/T.../B.../xxx
```

#### List all destinations

```bash
pesn destination list
```

#### Show destination details

```bash
pesn destination show --name <name>
```

#### Remove a destination

```bash
pesn destination remove --name <name>
```

> A destination cannot be removed while it is in use by a monitored process.

---

### 2. Managing monitored processes

#### Add a process to monitor

```bash
pesn process add --pid <pid> --destination <name> [--memo <memo>]
```

| Option          | Short | Required | Description |
|-----------------|-------|----------|-------------|
| `--pid`         | `-p`  | Yes | PID of the process to monitor |
| `--destination` | `-d`  | Yes | Name of the destination (must already exist) |
| `--memo`        | `-m`  | No  | Optional note |

At registration time, `pesn` reads `/proc/{pid}/cmdline` and `/proc/{pid}/cwd` to capture the command line and working directory of the process.

**Example:**

```bash
# Monitor an ML training job
pesn process add -p 12345 -d my-channel -m "ML training job"
```

#### List all monitored processes

```bash
pesn process list
```

#### Show details of a monitored process

```bash
pesn process show --pid <pid>
```

#### Remove a process from monitoring

```bash
pesn process remove --pid <pid>
```

---

### 3. Check for terminated processes

```bash
pesn check
```

Checks all monitored processes and sends a Slack notification for any that have terminated. Notified processes are automatically removed from the monitoring list.

#### Example cron job (check every minute)

```cron
* * * * * /path/to/pesn check
```

---

## Slack notification contents

When a process termination is detected, the following information is included in the notification:

- `<!channel>` mention + detection time (JST)
- **PID** — process ID
- **Command** — command line captured at registration
- **Cwd** — working directory captured at registration
- **Memo** — optional note (only if set)

---

## Configuration files

Configuration is stored automatically in TOML files under `~/.config/pesn/`. The files are created on first use.

| File | Contents |
|------|----------|
| `~/.config/pesn/config.toml` | Destination definitions |
| `~/.config/pesn/processes.toml` | List of monitored processes |

### `~/.config/pesn/config.toml` example

```toml
[[destination]]
name = "my-channel"
url = "https://hooks.slack.com/services/T.../B.../xxx"
memo = "optional memo"
```

### `~/.config/pesn/processes.toml` example

```toml
[[process]]
pid = 12345
cwd = "/home/user/project"
command = "python train.py --epochs 100"
destination = "my-channel"
memo = "ML training job"
```

---

## License

MIT
