cask "ai-timeblock-calendar" do
  version "0.1.0"
  sha256 "REPLACE_WITH_SHA256"

  url "https://github.com/example/ai-timeblock-calendar/releases/download/v#{version}/ai-timeblock-calendar-macos-aarch64.dmg"
  name "AI Time Block Calendar"
  desc "Desktop app for AI-assisted time blocking and Google Calendar sync"
  homepage "https://github.com/example/ai-timeblock-calendar"

  app "AI Time Block Calendar.app"

  zap trash: [
    "~/Library/Application Support/ai-timeblock-calendar",
    "~/Library/Preferences/com.aitimeblock.calendar.plist"
  ]
end
