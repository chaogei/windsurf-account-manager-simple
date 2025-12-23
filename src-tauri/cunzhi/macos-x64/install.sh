#!/bin/bash
#
# Windsurf Cunzhi MCP - Installation Script for macOS/Linux
#
# Usage:
#   ./install.sh              # Install with default settings
#   ./install.sh --no-build   # Skip build, use pre-compiled binaries
#   ./install.sh --uninstall  # Uninstall
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# Functions
info() { echo -e "${CYAN}[INFO]${NC} $1"; }
ok() { echo -e "${GREEN}[OK]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Darwin*)    OS="macos" ;;
        Linux*)     OS="linux" ;;
        *)          error "Unsupported OS: $(uname -s)"; exit 1 ;;
    esac
    info "Detected OS: $OS"
}

# Set paths based on OS
set_paths() {
    case "$OS" in
        macos)
            INSTALL_DIR="$HOME/Library/Application Support/windsurf-cunzhi"
            CONFIG_DIR="$HOME/.codeium/windsurf"
            MCP_CONFIG="$CONFIG_DIR/mcp_config.json"
            GLOBAL_RULES="$CONFIG_DIR/memories/global_rules.md"
            ;;
        linux)
            INSTALL_DIR="$HOME/.local/share/windsurf-cunzhi"
            CONFIG_DIR="$HOME/.codeium/windsurf"
            MCP_CONFIG="$CONFIG_DIR/mcp_config.json"
            GLOBAL_RULES="$CONFIG_DIR/memories/global_rules.md"
            ;;
    esac
}

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Parse arguments
NO_BUILD=false
UNINSTALL=false
BUILD_TAURI=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --no-build)
            NO_BUILD=true
            shift
            ;;
        --build-tauri)
            BUILD_TAURI=true
            shift
            ;;
        --uninstall)
            UNINSTALL=true
            shift
            ;;
        *)
            error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Header
echo ""
echo -e "${MAGENTA}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${MAGENTA}║       Windsurf Cunzhi MCP - Installer (macOS/Linux)        ║${NC}"
echo -e "${MAGENTA}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

detect_os
set_paths

# Uninstall
if [ "$UNINSTALL" = true ]; then
    info "Uninstalling Windsurf Cunzhi..."
    
    # Remove installation directory
    if [ -d "$INSTALL_DIR" ]; then
        rm -rf "$INSTALL_DIR"
        ok "Removed installation directory"
    fi
    
    # Remove MCP config entry
    if [ -f "$MCP_CONFIG" ]; then
        if command -v jq &> /dev/null; then
            jq 'del(.mcpServers["windsurf-cunzhi"])' "$MCP_CONFIG" > "${MCP_CONFIG}.tmp" && mv "${MCP_CONFIG}.tmp" "$MCP_CONFIG"
            ok "Removed MCP configuration"
        else
            warn "jq not installed, please manually remove windsurf-cunzhi from $MCP_CONFIG"
        fi
    fi
    
    # Remove global rules marker
    if [ -f "$GLOBAL_RULES" ]; then
        if grep -q "最高优先级规则 - 寸止" "$GLOBAL_RULES"; then
            # Remove the cunzhi rules section
            sed -i.bak '/# ⚠️ 最高优先级规则 - 寸止/,/^---$/d' "$GLOBAL_RULES"
            rm -f "${GLOBAL_RULES}.bak"
            ok "Removed global rules"
        fi
    fi
    
    ok "Uninstallation complete!"
    exit 0
fi

# Build MCP server
build_mcp() {
    info "Building MCP server..."
    
    if ! command -v cargo &> /dev/null; then
        error "Cargo not found. Please install Rust: https://rustup.rs/"
        exit 1
    fi
    
    cd "$SCRIPT_DIR"
    cargo build --release
    ok "MCP server built successfully"
}

# Build Tauri UI
build_tauri() {
    info "Building Tauri UI..."
    
    if ! command -v npm &> /dev/null; then
        error "npm not found. Please install Node.js"
        exit 1
    fi
    
    cd "$SCRIPT_DIR"
    npm install
    npm run tauri build
    ok "Tauri UI built successfully"
}

# Install files
install_files() {
    info "Installing files to $INSTALL_DIR..."
    
    mkdir -p "$INSTALL_DIR"
    
    # Copy MCP server
    if [ -f "$SCRIPT_DIR/target/release/windsurf-cunzhi" ]; then
        cp "$SCRIPT_DIR/target/release/windsurf-cunzhi" "$INSTALL_DIR/"
        chmod +x "$INSTALL_DIR/windsurf-cunzhi"
        ok "Installed MCP server"
    else
        error "MCP server binary not found. Run without --no-build to compile."
        exit 1
    fi
    
    # Copy Tauri UI
    case "$OS" in
        macos)
            if [ -d "$SCRIPT_DIR/src-tauri/target/release/bundle/macos/windsurf-cunzhi-ui.app" ]; then
                cp -r "$SCRIPT_DIR/src-tauri/target/release/bundle/macos/windsurf-cunzhi-ui.app" "$INSTALL_DIR/"
                ok "Installed Tauri UI (macOS app bundle)"
            elif [ -f "$SCRIPT_DIR/src-tauri/target/release/windsurf-cunzhi-ui" ]; then
                cp "$SCRIPT_DIR/src-tauri/target/release/windsurf-cunzhi-ui" "$INSTALL_DIR/"
                chmod +x "$INSTALL_DIR/windsurf-cunzhi-ui"
                ok "Installed Tauri UI"
            else
                warn "Tauri UI not found. Run with --build-tauri to compile."
            fi
            ;;
        linux)
            if [ -f "$SCRIPT_DIR/src-tauri/target/release/windsurf-cunzhi-ui" ]; then
                cp "$SCRIPT_DIR/src-tauri/target/release/windsurf-cunzhi-ui" "$INSTALL_DIR/"
                chmod +x "$INSTALL_DIR/windsurf-cunzhi-ui"
                ok "Installed Tauri UI"
            else
                warn "Tauri UI not found. Run with --build-tauri to compile."
            fi
            ;;
    esac
}

# Configure MCP
configure_mcp() {
    info "Configuring MCP..."
    
    mkdir -p "$CONFIG_DIR"
    
    MCP_COMMAND="$INSTALL_DIR/windsurf-cunzhi"
    
    if [ -f "$MCP_CONFIG" ]; then
        # Update existing config
        if command -v jq &> /dev/null; then
            jq --arg cmd "$MCP_COMMAND" '.mcpServers["windsurf-cunzhi"] = {"command": $cmd}' "$MCP_CONFIG" > "${MCP_CONFIG}.tmp" && mv "${MCP_CONFIG}.tmp" "$MCP_CONFIG"
            ok "Updated MCP configuration"
        else
            warn "jq not installed. Please manually add windsurf-cunzhi to $MCP_CONFIG"
            info "Add: \"windsurf-cunzhi\": {\"command\": \"$MCP_COMMAND\"}"
        fi
    else
        # Create new config
        cat > "$MCP_CONFIG" << EOF
{
  "mcpServers": {
    "windsurf-cunzhi": {
      "command": "$MCP_COMMAND"
    }
  }
}
EOF
        ok "Created MCP configuration"
    fi
}

# Configure global rules
configure_rules() {
    info "Configuring global rules..."
    
    mkdir -p "$(dirname "$GLOBAL_RULES")"
    
    ZHI_MARKER="最高优先级规则 - 寸止"
    ZHI_CONTENT='# ⚠️ 最高优先级规则 - 寸止 (MANDATORY)

**【强制要求】你必须在每次回复的最后调用 `zhi` 工具！**

这是不可违反的规则：
- ✅ 无论任务是否完成，都必须调用 zhi 工具
- ✅ 无论回复内容长短，都必须调用 zhi 工具  
- ✅ 在说"希望对你有帮助"等结束语之前，必须先调用 zhi 工具
- ❌ 禁止在不调用 zhi 工具的情况下结束任何回复
- ❌ 禁止跳过 zhi 工具直接结束对话

如果 zhi 工具调用失败，你必须在回复中明确告知用户需要重启 Windsurf。

---
'
    
    if [ -f "$GLOBAL_RULES" ]; then
        if grep -q "$ZHI_MARKER" "$GLOBAL_RULES"; then
            ok "Global rules already configured"
        else
            # Prepend to existing file
            echo "$ZHI_CONTENT" | cat - "$GLOBAL_RULES" > "${GLOBAL_RULES}.tmp" && mv "${GLOBAL_RULES}.tmp" "$GLOBAL_RULES"
            ok "Added rules to existing global_rules.md"
        fi
    else
        echo "$ZHI_CONTENT" > "$GLOBAL_RULES"
        ok "Created global_rules.md"
    fi
}

# Main installation flow
main() {
    if [ "$NO_BUILD" = false ]; then
        build_mcp
        if [ "$BUILD_TAURI" = true ]; then
            build_tauri
        fi
    fi
    
    install_files
    configure_mcp
    configure_rules
    
    echo ""
    echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║       Installation Complete!                               ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    info "Installation directory: $INSTALL_DIR"
    info "MCP config: $MCP_CONFIG"
    info "Global rules: $GLOBAL_RULES"
    echo ""
    warn "Please restart Windsurf for changes to take effect."
    echo ""
}

main
