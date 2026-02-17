#!/usr/bin/env node
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

const command = process.argv[2] ?? 'help';
const args = new Set(process.argv.slice(3));

const installRoot = path.join(
  os.homedir(),
  '.local',
  'share',
  'ai-timeblock-calendar'
);
const binDir = path.join(installRoot, 'bin');
const mcpBinary = path.join(binDir, 'mcp-server');
const appMarker = path.join(installRoot, 'app.installed');
const claudeConfig = path.join(
  os.homedir(),
  '.config',
  'Claude',
  'claude_desktop_config.json'
);

function ensureDir(dirPath) {
  fs.mkdirSync(dirPath, { recursive: true });
}

function detectTarget() {
  const platform = os.platform();
  const arch = os.arch();
  return `${platform}-${arch}`;
}

function releaseBaseUrl() {
  const repo =
    process.env.AI_TIMEBLOCK_REPO ??
    'https://github.com/YuminosukeSato/ai-timeblock-calendar';
  const version = process.env.AI_TIMEBLOCK_VERSION ?? 'v0.1.0';
  return `${repo}/releases/download/${version}`;
}

function registerClaudeConfig() {
  ensureDir(path.dirname(claudeConfig));
  let json = {};
  if (fs.existsSync(claudeConfig)) {
    try {
      json = JSON.parse(fs.readFileSync(claudeConfig, 'utf8'));
    } catch {
      json = {};
    }
  }

  if (!json.mcpServers || typeof json.mcpServers !== 'object') {
    json.mcpServers = {};
  }

  json.mcpServers['ai-timeblock-calendar'] = {
    command: mcpBinary,
    args: []
  };

  fs.writeFileSync(claudeConfig, `${JSON.stringify(json, null, 2)}\n`);
}

function install() {
  const withGui =
    args.has('--with-gui') ||
    (!args.has('--with-mcp') && !args.has('--without-gui'));
  const withMcp =
    args.has('--with-mcp') ||
    (!args.has('--with-gui') && !args.has('--without-mcp'));

  ensureDir(binDir);

  const target = detectTarget();
  const base = releaseBaseUrl();

  if (withMcp) {
    const mcpUrl = `${base}/mcp-server-${target}.tar.gz`;
    fs.writeFileSync(
      mcpBinary,
      `#!/bin/sh\necho "stub mcp-server from ${mcpUrl}"\n`
    );
    fs.chmodSync(mcpBinary, 0o755);
    console.log(`Installed MCP binary stub: ${mcpBinary}`);
    console.log(`Download URL (to be wired in CI): ${mcpUrl}`);
  }

  if (withGui) {
    const appUrl = `${base}/ai-timeblock-calendar-${target}.tar.gz`;
    fs.writeFileSync(appMarker, `GUI marker for ${appUrl}\n`);
    console.log(`Installed GUI marker: ${appMarker}`);
    console.log(`Download URL (to be wired in CI): ${appUrl}`);
  }

  if (!args.has('--no-config')) {
    registerClaudeConfig();
    console.log(`Updated Claude config: ${claudeConfig}`);
  }
}

function doctor() {
  const checks = [
    { name: 'install root', ok: fs.existsSync(installRoot) },
    { name: 'mcp binary', ok: fs.existsSync(mcpBinary) },
    { name: 'claude config', ok: fs.existsSync(claudeConfig) }
  ];

  checks.forEach((check) => {
    console.log(`${check.ok ? 'OK' : 'NG'}: ${check.name}`);
  });

  const failed = checks.some((check) => !check.ok);
  process.exitCode = failed ? 1 : 0;
}

function uninstall() {
  if (fs.existsSync(installRoot)) {
    fs.rmSync(installRoot, { recursive: true, force: true });
  }
  console.log(`Removed: ${installRoot}`);
}

switch (command) {
  case 'install':
    install();
    break;
  case 'doctor':
    doctor();
    break;
  case 'uninstall':
    uninstall();
    break;
  default:
    console.log(
      'Usage: ai-timeblock <install|doctor|uninstall> [--with-gui] [--with-mcp] [--no-config]'
    );
}
