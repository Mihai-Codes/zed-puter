# zed-puter

[![CI](https://github.com/Mihai-Codes/zed-puter/actions/workflows/ci.yml/badge.svg)](https://github.com/Mihai-Codes/zed-puter/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/Mihai-Codes/zed-puter/graph/badge.svg)](https://codecov.io/gh/Mihai-Codes/zed-puter)
[![Zed Extension](https://img.shields.io/badge/Zed-Extension-blue)](https://zed.dev/extensions/puter)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![MCP](https://img.shields.io/badge/MCP-Model%20Context%20Protocol-purple)](https://modelcontextprotocol.io/)

> 🚧 **Work in Progress** - This extension is under active development.

Zed IDE extension for [Puter.com](https://puter.com) AI models. Access Claude, GPT, Gemini, DeepSeek, and 500+ more AI models directly in Zed's Agent Panel.

## Features

- **500+ AI Models** - Claude Opus 4.5, GPT-5.2, Gemini 2.5 Pro, DeepSeek R1, and more
- **400+ FREE Models** - OpenRouter free tier models with no cost
- **Automatic Fallback** - When premium models are rate-limited, falls back to free alternatives
- **No API Keys** - Just sign in with your Puter account
- **Free Tier** - Try before you buy with Puter's free credits

## Installation

### From Zed Extensions (Coming Soon)

1. Open Zed
2. Press `Ctrl+Shift+X` (or `Cmd+Shift+X` on macOS) to open Extensions
3. Search for "Puter"
4. Click Install

### Manual Installation (Dev Mode)

1. Clone this repository
2. In Zed, open the command palette (`Ctrl+Shift+P` or `Cmd+Shift+P`)
3. Run `zed: install dev extension`
4. Select the `zed-puter` directory

## Prerequisites

Before using this extension, you need to authenticate with Puter:

```bash
npx opencode-puter-auth login
```

This opens a browser window to sign in with your Puter account.

## Configuration

Add to your Zed settings (`~/.config/zed/settings.json`):

```json
{
  "context_servers": {
    "puter": {
      "settings": {
        "default_model": "claude-opus-4-5",
        "fallback_enabled": true,
        "debug": false
      }
    }
  }
}
```

### Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `default_model` | string | `"claude-opus-4-5"` | Default AI model to use |
| `fallback_enabled` | boolean | `true` | Fall back to free models when rate limited |
| `debug` | boolean | `false` | Enable debug logging |

## Available Models

### Premium Models (via Puter)

| Model ID | Description |
|----------|-------------|
| `claude-opus-4-5` | Best coding model in the world |
| `claude-sonnet-4-5` | Balanced performance |
| `gpt-5.2` | Latest OpenAI model |
| `google/gemini-2.5-pro` | 1M context window |
| `deepseek-r1` | Advanced reasoning |

### Free Models (via OpenRouter)

| Model ID | Description |
|----------|-------------|
| `openrouter:xiaomi/mimo-v2-flash:free` | #1 on SWE-bench |
| `openrouter:deepseek/deepseek-r1-0528:free` | o1-level reasoning |
| `openrouter:mistralai/devstral-2512:free` | Agentic coding |
| `openrouter:qwen/qwen3-coder:free` | 480B coding model |

## MCP Tools

This extension provides the following MCP tools to Zed's Agent:

- **puter-chat** - Chat with AI models through Puter.com
- **puter-models** - List all available AI models
- **puter-account** - Show current account information

## Development

### Prerequisites

- Rust (via rustup)
- Node.js 20+

### Building

```bash
# Build the extension
cargo build

# For release
cargo build --release
```

### Testing Locally

1. Build the extension
2. In Zed, run `zed: install dev extension`
3. Select this directory

## Related Projects

- [opencode-puter-auth](https://github.com/Mihai-Codes/opencode-puter-auth) - OpenCode plugin for Puter AI models

## License

MIT - See [LICENSE](LICENSE)

---

**Made with ❤️ by [@chindris-mihai-alexandru](https://github.com/chindris-mihai-alexandru)**
