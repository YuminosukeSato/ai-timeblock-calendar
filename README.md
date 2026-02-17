# AI Time Block Calendar

Rust中心のデスクトップカレンダーアプリです。  
Tauri + React UI でタイムブロック管理を行い、MCPサーバーと共有SQLiteを通じてAIスケジューリングを実行します。

## Local Development

```bash
cargo test --workspace
pnpm install
pnpm test
pnpm lint
pnpm format
pnpm format:check
```

## Lint and Formatter

```bash
# TypeScript + Rust lint
pnpm lint

# TypeScript only
pnpm lint:ts

# Rust only
pnpm lint:rust

# Apply formatter
pnpm format

# Check formatter without writing
pnpm format:check
```

## Distribution

### npm installer

```bash
npm i -g @aitimeblock/ai-timeblock-installer
ai-timeblock install --with-gui --with-mcp
ai-timeblock doctor
```

### Homebrew

```bash
brew tap example/ai-timeblock
brew install --cask ai-timeblock-calendar
brew install ai-timeblock-mcp
```

## Architecture

```text
Claude/Codex --stdio--> mcp-server (Rust)
                             |
                             v
                     shared SQLite (WAL)
                             ^
                             |
React UI --invoke--> src-tauri commands (Rust)
```
