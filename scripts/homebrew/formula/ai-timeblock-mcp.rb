class AiTimeblockMcp < Formula
  desc "MCP server for AI Time Block Calendar"
  homepage "https://github.com/example/ai-timeblock-calendar"
  version "0.1.0"
  url "https://github.com/example/ai-timeblock-calendar/releases/download/v#{version}/mcp-server-macos-aarch64.tar.gz"
  sha256 "REPLACE_WITH_SHA256"

  def install
    bin.install "mcp-server"
  end

  test do
    assert_match "mcp-server", shell_output("#{bin}/mcp-server --help", 0)
  end
end
