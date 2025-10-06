# AIChat: All-in-one LLM Platform

[![CI](https://github.com/sigoden/aichat/actions/workflows/ci.yaml/badge.svg)](https://github.com/sigoden/aichat/actions/workflows/ci.yaml)
[![Crates](https://img.shields.io/crates/v/aichat.svg)](https://crates.io/crates/aichat)
[![Discord](https://img.shields.io/discord/1226737085453701222?label=Discord)](https://discord.gg/mr3ZZUB9hG)

AIChat is an all-in-one LLM platform featuring a powerful CLI tool with Shell Assistant, CMD & REPL modes, RAG, AI Tools & Agents, and a comprehensive web interface with Matrix theming, API marketplace, document processing, and cloud integrations.

## 🌟 What's New

**AIChat Web Platform Transformation** - AIChat is evolving from a CLI-only tool into a full-featured web platform while maintaining all CLI capabilities. The transformation includes:

- 🎨 **Matrix-Themed Web Interface** - Immersive cyberpunk UI with animated rain effects
- 🔐 **User Authentication & Sessions** - JWT-based auth with persistent sessions
- 🏪 **API Key Marketplace** - Buy/sell API access with dynamic pricing & intelligent routing
- 💳 **Integrated Billing System** - Stripe payments, balance tracking, and automated payouts
- 🖥️ **Web Terminal Emulator** - Full CLI experience in browser via xterm.js
- 🔄 **Multi-Provider Chat** - Side-by-side model comparison with real-time cost tracking
- ☁️ **Cloud Document Processing** - Google Drive & OneDrive integration with RAG
- 🔍 **Web Search Integration** - DuckDuckGo & Google Search API with citations
- 📊 **Analytics Dashboard** - Usage metrics, cost analysis, and PDF reports

**Current Status:** Foundation complete (13%) - Authentication system and Matrix theme operational. [View Epic Progress →](https://github.com/johnproblems/aichat/issues/1)

## 🚀 Key Features

### CLI Power User Features
- **20+ LLM Providers** - OpenAI, Claude, Gemini, Ollama, and more in one tool
- **Shell Assistant** - Natural language to shell commands
- **Interactive REPL** - Tab completion, history search, multi-line input
- **RAG Integration** - Chat with your documents and codebase
- **AI Agents** - Build custom GPT-like agents with tools and knowledge
- **Function Calling** - Connect LLMs to external APIs and services
- **Session Management** - Persistent conversations with context

### Web Platform Exclusive Features
- **API Marketplace** - Monetize your API keys or find cheaper alternatives
  - Dynamic pricing with real-time cost calculations
  - Intelligent routing with automatic failover
  - Capacity tracking and usage limits
  - Revenue sharing with automated payouts

- **Multi-Provider Arena** - Compare LLMs side-by-side
  - Simultaneous queries to multiple providers
  - Real-time cost comparison per response
  - Response quality voting and analytics
  - Export conversation comparisons

- **Cloud Document Hub** - Unified document processing
  - Google Drive and OneDrive integration
  - Automatic RAG processing for uploaded files
  - Citation-aware responses with source links
  - Collaborative document discussions

- **Analytics & Insights** - Track everything
  - Usage metrics across all providers
  - Cost analysis and spending trends
  - Performance monitoring dashboards
  - PDF report generation and scheduling

- **Matrix-Themed Experience** - Cyberpunk aesthetics
  - Animated code rain effects
  - Terminal-style interface design
  - Green phosphor text styling
  - Retro-futuristic UI elements

## 📦 Install

### Package Managers

- **Rust Developers:** `cargo install aichat`
- **Homebrew/Linuxbrew Users:** `brew install aichat`
- **Pacman Users**: `pacman -S aichat`
- **Windows Scoop Users:** `scoop install aichat`
- **Android Termux Users:** `pkg install aichat`

### Pre-built Binaries

Download pre-built binaries for macOS, Linux, and Windows from [GitHub Releases](https://github.com/sigoden/aichat/releases), extract them, and add the `aichat` binary to your `$PATH`.

## ⚡ Quick Start

### CLI Usage
```bash
# Set your API key (OpenAI example)
export OPENAI_API_KEY="sk-..."

# Ask a question
aichat "Explain Rust ownership"

# Start interactive REPL
aichat

# Use shell assistant
aichat --execute "find all rust files modified today"

# Chat with documents (RAG)
aichat --file docs/ "Summarize the architecture"

# Use a specific model
aichat --model claude:claude-3-5-sonnet "Write a poem"
```

### Web Platform Usage
```bash
# Start the web server
aichat --serve

# Or on a specific port
aichat --serve 3000

# Access the platform
# 1. Open http://localhost:8000 in your browser
# 2. Create an account or log in
# 3. Explore the Matrix-themed interface
# 4. Try the marketplace, multi-provider chat, or terminal
```

### First-Time Setup
```bash
# Configure your LLM providers
aichat --list-models        # View available models
aichat --model openai:gpt-4 # Set default model

# Create a custom role
aichat --role .edit         # Opens editor for new role

# Set up an AI agent
aichat --agent my-agent .edit   # Configure agent with tools & docs
```

## 🎯 Features

### Multi-Providers

Integrate seamlessly with over 20 leading LLM providers through a unified interface. Supported providers include OpenAI, Claude, Gemini (Google AI Studio), Ollama, Groq, Azure-OpenAI, VertexAI, Bedrock, Github Models, Mistral, Deepseek, AI21, XAI Grok, Cohere, Perplexity, Cloudflare, OpenRouter, Ernie, Qianwen, Moonshot, ZhipuAI, MiniMax, Deepinfra, VoyageAI, any OpenAI-Compatible API provider.

### CMD Mode

Explore powerful command-line functionalities with AIChat's CMD mode.

![aichat-cmd](https://github.com/user-attachments/assets/6c58c549-1564-43cf-b772-e1c9fe91d19c)

### REPL Mode

Experience an interactive Chat-REPL with features like tab autocompletion, multi-line input support, history search, configurable keybindings, and custom REPL prompts.

![aichat-repl](https://github.com/user-attachments/assets/218fab08-cdae-4c3b-bcf8-39b6651f1362)

### Shell Assistant

Elevate your command-line efficiency. Describe your tasks in natural language, and let AIChat transform them into precise shell commands. AIChat intelligently adjusts to your OS and shell environment.

![aichat-execute](https://github.com/user-attachments/assets/0c77e901-0da2-4151-aefc-a2af96bbb004)

### Multi-Form Input

Accept diverse input forms such as stdin, local files and directories, and remote URLs, allowing flexibility in data handling.

| Input             | CMD                                  | REPL                             |
| ----------------- | ------------------------------------ | -------------------------------- |
| CMD               | `aichat hello`                       |                                  |
| STDIN             | `cat data.txt \| aichat`             |                                  |
| Last Reply        |                                      | `.file %%`                       |
| Local files       | `aichat -f image.png -f data.txt`    | `.file image.png data.txt`       |
| Local directories | `aichat -f dir/`                     | `.file dir/`                     |
| Remote URLs       | `aichat -f https://example.com`      | `.file https://example.com`      |
| External commands | ```aichat -f '`git diff`'```         | ```.file `git diff` ```          |
| Combine Inputs    | `aichat -f dir/ -f data.txt explain` | `.file dir/ data.txt -- explain` |

### Role

Customize roles to tailor LLM behavior, enhancing interaction efficiency and boosting productivity.

![aichat-role](https://github.com/user-attachments/assets/023df6d2-409c-40bd-ac93-4174fd72f030)

> The role consists of a prompt and model configuration.

### Session

Maintain context-aware conversations through sessions, ensuring continuity in interactions.

![aichat-session](https://github.com/user-attachments/assets/56583566-0f43-435f-95b3-730ae55df031)

> The left side uses a session, while the right side does not use a session.

### Macro

Streamline repetitive tasks by combining a series of REPL commands into a custom macro.

![aichat-macro](https://github.com/user-attachments/assets/23c2a08f-5bd7-4bf3-817c-c484aa74a651)

### RAG

Integrate external documents into your LLM conversations for more accurate and contextually relevant responses.

![aichat-rag](https://github.com/user-attachments/assets/359f0cb8-ee37-432f-a89f-96a2ebab01f6)

### Function Calling

Function calling supercharges LLMs by connecting them to external tools and data sources. This unlocks a world of possibilities, enabling LLMs to go beyond their core capabilities and tackle a wider range of tasks.

We have created a new repository [https://github.com/sigoden/llm-functions](https://github.com/sigoden/llm-functions) to help you make the most of this feature.

#### AI Tools & MCP

Integrate external tools to automate tasks, retrieve information, and perform actions directly within your workflow.

![aichat-tool](https://github.com/user-attachments/assets/7459a111-7258-4ef0-a2dd-624d0f1b4f92)

#### AI Agents (CLI version of OpenAI GPTs)

AI Agent = Instructions (Prompt) + Tools (Function Callings) + Documents (RAG).

![aichat-agent](https://github.com/user-attachments/assets/0b7e687d-e642-4e8a-b1c1-d2d9b2da2b6b)

### Local Server & Web Platform

AIChat includes a powerful built-in HTTP server that serves both APIs and the web interface.

```bash
$ aichat --serve
Chat Completions API: http://127.0.0.1:8000/v1/chat/completions
Embeddings API:       http://127.0.0.1:8000/v1/embeddings
Rerank API:           http://127.0.0.1:8000/v1/rerank
LLM Playground:       http://127.0.0.1:8000/playground
LLM Arena:            http://127.0.0.1:8000/arena?num=2
Web Platform:         http://127.0.0.1:8000/           (Matrix GUI)
```

**Web Platform Features:**
- **Matrix Console Interface** - Cyberpunk-themed UI with green code rain animation
- **Authentication Portal** - Secure login/signup with JWT tokens
- **API Marketplace** - Browse, purchase, and sell API key access
- **Web Terminal** - Full-featured terminal emulator with xterm.js
- **Multi-Provider Chat** - Compare responses from multiple LLMs side-by-side
- **Document Manager** - Upload from cloud storage (Google Drive, OneDrive)
- **Analytics Dashboard** - Track usage, costs, and performance metrics
- **Billing Portal** - Manage payments, balance, and transactions

#### Proxy LLM APIs

The LLM Arena is a web-based platform where you can compare different LLMs side-by-side. 

Test with curl:

```sh
curl -X POST -H "Content-Type: application/json" -d '{
  "model":"claude:claude-3-5-sonnet-20240620",
  "messages":[{"role":"user","content":"hello"}], 
  "stream":true
}' http://127.0.0.1:8000/v1/chat/completions
```

#### LLM Playground

A web application to interact with supported LLMs directly from your browser.

![aichat-llm-playground](https://github.com/user-attachments/assets/aab1e124-1274-4452-b703-ef15cda55439)

#### LLM Arena

A web platform to compare different LLMs side-by-side.

![aichat-llm-arena](https://github.com/user-attachments/assets/edabba53-a1ef-4817-9153-38542ffbfec6)

## Custom Themes

AIChat supports custom dark and light themes, which highlight response text and code blocks.

![aichat-themes](https://github.com/sigoden/aichat/assets/4012553/29fa8b79-031e-405d-9caa-70d24fa0acf8)

## Documentation

### CLI Documentation
- [Chat-REPL Guide](https://github.com/sigoden/aichat/wiki/Chat-REPL-Guide)
- [Command-Line Guide](https://github.com/sigoden/aichat/wiki/Command-Line-Guide)
- [Role Guide](https://github.com/sigoden/aichat/wiki/Role-Guide)
- [Macro Guide](https://github.com/sigoden/aichat/wiki/Macro-Guide)
- [RAG Guide](https://github.com/sigoden/aichat/wiki/RAG-Guide)
- [Environment Variables](https://github.com/sigoden/aichat/wiki/Environment-Variables)
- [Configuration Guide](https://github.com/sigoden/aichat/wiki/Configuration-Guide)
- [Custom Theme](https://github.com/sigoden/aichat/wiki/Custom-Theme)
- [Custom REPL Prompt](https://github.com/sigoden/aichat/wiki/Custom-REPL-Prompt)
- [FAQ](https://github.com/sigoden/aichat/wiki/FAQ)

### Web Platform Documentation
- [Web Platform Architecture](.claude/epics/aichat/epic.md) - Technical overview and architecture
- [API Marketplace Guide](https://github.com/johnproblems/aichat/issues/2) - Buy/sell API keys
- [Web Terminal Usage](https://github.com/johnproblems/aichat/issues/7) - Browser-based CLI
- [Multi-Provider Chat](https://github.com/johnproblems/aichat/issues/12) - Model comparison
- [Document Processing](https://github.com/johnproblems/aichat/issues/17) - Cloud integrations
- [Billing & Payments](https://github.com/johnproblems/aichat/issues/27) - Stripe integration
- [Analytics Dashboard](https://github.com/johnproblems/aichat/issues/32) - Usage metrics

## Architecture

AIChat is built in Rust with a modular architecture:

### Core Components
- **CLI Interface** (`src/cli.rs`) - Clap-based command parsing
- **Client System** (`src/client/`) - Multi-provider LLM integration (20+ providers)
- **Server** (`src/serve.rs`) - Hyper-based HTTP/WebSocket server
- **Configuration** (`src/config/`) - Roles, sessions, agents management
- **RAG System** (`src/rag/`) - Document processing and vector search
- **REPL** (`src/repl/`) - Interactive chat with syntax highlighting
- **Function Calling** (`src/function.rs`) - External tool integration

### Web Platform Stack
- **Backend:** Rust (Hyper, Tokio, Serde)
- **Database:** PostgreSQL/Supabase
- **Frontend:** Matrix-themed HTML/CSS/JS with WebSockets
- **Terminal:** xterm.js with WebSocket backend
- **Payments:** Stripe API integration
- **Cloud Storage:** Google Drive & OneDrive APIs
- **Search:** DuckDuckGo & Google Custom Search

### Development Roadmap

**Phase 1: Foundation ✅ (Complete)**
- Matrix-themed web interface with animated code rain
- PostgreSQL database schema and migrations
- JWT authentication and user management system

**Phase 2: Marketplace & Payments 🚧 (In Progress)**
- API key marketplace with listing management
- Dynamic pricing engine and capacity tracking
- Intelligent request routing with failover
- Stripe payment integration and billing system
- Balance management and automated payouts

**Phase 3: Enhanced UX 📋 (Planned)**
- xterm.js terminal emulator with WebSocket backend
- Side-by-side multi-provider chat comparison
- Real-time cost tracking and analysis
- Google Drive & OneDrive cloud integration
- Document processing pipeline with RAG

**Phase 4: Intelligence & Insights 📋 (Planned)**
- DuckDuckGo & Google Search integration
- Web search with citation extraction
- Comprehensive analytics dashboard
- Usage metrics and PDF report generation
- System-wide performance monitoring

### Current Development Status
Track progress in [Epic Issue #1](https://github.com/johnproblems/aichat/issues/1) and individual task issues:
- ✅ Task 1: Matrix Console Theme Foundation (Complete)
- ✅ Task 2: Database Schema & User Management (Complete)
- 🚧 Task 3: API Key Marketplace Core ([#2-6](https://github.com/johnproblems/aichat/issues/2))
- 🚧 Task 4: Terminal Emulator ([#7-11](https://github.com/johnproblems/aichat/issues/7))
- 🚧 Task 5: Multi-Provider Chat ([#12-16](https://github.com/johnproblems/aichat/issues/12))
- 🚧 Task 6: Document Processing ([#17-21](https://github.com/johnproblems/aichat/issues/17))
- 🚧 Task 7: Web Search Integration ([#22-26](https://github.com/johnproblems/aichat/issues/22))
- 🚧 Task 8: Billing & Payment System ([#27-31](https://github.com/johnproblems/aichat/issues/27))
- 🚧 Task 9: Analytics & Reporting ([#32-36](https://github.com/johnproblems/aichat/issues/32))

**Total Progress:** 13% complete (2 of 9 core tasks) • 35 subtasks planned • ~15 weeks estimated

## Contributing

We welcome contributions! See our [Epic roadmap](https://github.com/johnproblems/aichat/issues/1) for areas where help is needed. Each feature has detailed subtasks with implementation guides.

### Development Setup
```bash
# Clone repository
git clone https://github.com/johnproblems/aichat.git
cd aichat

# Build project
cargo build

# Run tests
cargo test

# Start local server
cargo run -- --serve

# Access web platform
open http://localhost:8000
```

### Project Structure
```
aichat/
├── src/                    # Rust source code
│   ├── cli.rs             # CLI interface
│   ├── serve.rs           # HTTP/WebSocket server
│   ├── client/            # Multi-provider LLM clients
│   ├── config/            # Configuration management
│   ├── rag/               # Document processing
│   └── repl/              # Interactive REPL
├── assets/                # Web UI assets
│   ├── index.html         # Matrix-themed interface
│   ├── styles.css         # Cyberpunk styling
│   └── app.js             # Frontend logic
├── migrations/            # Database migrations
├── .claude/               # Claude Code project management
│   ├── epics/            # Epic and task tracking
│   └── prds/             # Product requirements
└── Cargo.toml            # Rust dependencies
```

## License

Copyright (c) 2023-2025 aichat-developers.

AIChat is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the LICENSE-APACHE and LICENSE-MIT files for license details.