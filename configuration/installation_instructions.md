# Puter AI Models - Installation

## Prerequisites

Before using Puter AI models in Zed, you need to authenticate with Puter.com.

## Step 1: Install and Authenticate

Open your terminal and run:

```bash
npx opencode-puter-auth login
```

This will open a browser window to sign in with your Puter account.

## Step 2: Verify Authentication

```bash
npx opencode-puter-auth status
```

You should see your account information if authentication was successful.

## What You Get

- **500+ AI Models** - Claude Opus 4.5, GPT-5.2, Gemini 2.5 Pro, DeepSeek R1, and more
- **400+ FREE Models** - OpenRouter free tier models with no cost
- **Automatic Fallback** - When premium models are rate-limited, falls back to free alternatives
- **No API Keys** - Just your Puter account

## Available Models

### Premium Models
- `claude-opus-4-5` - Best coding model
- `claude-sonnet-4-5` - Balanced performance
- `gpt-5.2` - Latest OpenAI model
- `google/gemini-2.5-pro` - 1M context window

### Free Models (via OpenRouter)
- `openrouter:xiaomi/mimo-v2-flash:free` - #1 on SWE-bench
- `openrouter:deepseek/deepseek-r1-0528:free` - o1-level reasoning
- `openrouter:mistralai/devstral-2512:free` - Agentic coding

## Need Help?

- [Documentation](https://github.com/Mihai-Codes/opencode-puter-auth)
- [Issues](https://github.com/Mihai-Codes/zed-puter/issues)
