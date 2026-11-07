#!/bin/sh
# $HOME/github.com/loicbourgois/gravitle/front/moonlighter/scripts/dev.sh
set -e
osascript <<'EOF'
tell application "iTerm2"
    tell current window
        tell current tab
            tell first session
                split horizontally with same profile
            end tell
            repeat with s in sessions
                tell s
                    write text "cd $HOME/github.com/loicbourgois/gravitle"
                    write text "clear"
                end tell
            end repeat
            tell first session
                write text "$HOME/github.com/loicbourgois/gravitle/front/moonlighter/scripts/dev_inner.sh"
            end tell
            select last session
            tell last session
                write text "open http://localhost:82/moonlighter/"
            end tell
        end tell
    end tell
end tell
EOF
