# Windsurf Account Manager - Simple

A Windsurf multi-account management desktop application developed with Tauri + Vue 3 + TypeScript, designed to manage multiple Windsurf accounts and provide features such as credit reset, billing inquiries, and one-click account switching.

> ⚠️ **Free Software Disclaimer**: This software is completely free. If you paid for it, you have been scammed!

## 📱 Community Groups

<p align="center">
  <img src="../../public/交流群.png" alt="WeChat QR Code" width="300">
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="../../public/QQ群.jpg" alt="QQ QR Code" width="300">
</p>

---

## 🖥️ Software Interface

<p align="center">
  <img src="../../public/主页.png" alt="Software Homepage" width="800">
</p>

---

## ✨ Features

### 🔐 Account Management
- ✅ **Add/Edit/Delete Accounts** - Full CRUD operations for accounts.
- ✅ **Group Management** - Supports custom groups to easily organize multiple accounts.
- ✅ **Tag System** - Add custom tags to accounts.
- ✅ **Real-time Status Display** - Shows plan type, credit balance, expiration date, etc.
- ✅ **Batch Operations** - Select multiple accounts for batch reset or deletion.
- ✅ **Secure Storage** - Uses AES-256-GCM encryption with keys stored in the system keychain.

### 💳 Credit Reset
- ✅ **One-click Credit Reset** - Implements credit reset via seat count update APIs.
- ✅ **Smart Seat Rotation** - Automatically rotates between 3, 4, and 5 seats.
- ✅ **Batch Reset** - Supports concurrent reset for multiple accounts (up to 5).
- ✅ **Team Batch Reset** - One-click reset for all members within a team.
- ✅ **Automatic Reset Schedule** - Set scheduled tasks for automatic credit resets.

### 👥 Team Management
- ✅ **View Team Members** - Lists all member information within the team.
- ✅ **Invite Members** - Invite new members to the team via email.
- ✅ **Remove Members** - Remove specific members from the team.
- ✅ **Team Quota Management** - Unified management of team member credits.

### 🔄 One-click Switch
- ✅ **Fast Account Switching** - Quickly switch to other Windsurf accounts.
- ✅ **Automatic Token Refresh** - Automatically uses `refresh_token` to obtain new `access_token`.
- ✅ **OAuth Callback Trigger** - Automatically completes login via the `windsurf://` protocol.
- ✅ **Machine ID Reset** - Resets device identifier to support multi-device use (requires admin privileges).

### 🔧 Seamless Switch Patch
- ✅ **Auto-detect Windsurf Path** - Automatically finds the Windsurf installation location.
- ✅ **Apply Patch with One Click** - Modifies `extension.js` for seamless switching.
- ✅ **Remove Timeout Limit** - Removes the 180s OAuth timeout restriction.
- ✅ **Auto-backup** - Automatically backs up original files before patching (keeps up to 3 versions).
- ✅ **Restore Original State** - One-click restoration from backup files.
- ✅ **Auto-restart Windsurf** - Automatically restarts Windsurf after applying the patch.

### 💰 Payment Related
- ✅ **Virtual Card Image** - Generates virtual credit card information for payment testing.
- ✅ **Custom BIN** - Supports setting custom BIN numbers or BIN ranges.
- ✅ **Privacy Payment Window** - Opens Stripe payment pages in an independent incognito browser window.
- ✅ **Alipay/WeChat Pay** - Supports domestic payment methods (Donations).

### 📊 Data Inquiries
- ✅ **Billing Information** - Query plan, quota, usage, etc.
- ✅ **Subscription Status** - Displays subscription type, expiry date, and next billing date.
- ✅ **Usage Statistics** - View credit usage and remaining quota.
- ✅ **Global Refresh** - One-click update for all account information.

### ⚙️ System Settings
- ✅ **Proxy Configuration** - Supports HTTP proxy settings.
- ✅ **Lightweight API Mode** - Uses `GetPlanStatus` instead of `GetCurrentUser` to reduce requests.
- ✅ **Detailed Results Display** - Option to show detailed API response information.
- ✅ **Operation Logs** - Records all operation history, supports exporting.

### 🔒 Data Security
- ✅ **System Keychain** - Stores encryption keys in Windows Credential Manager.
- ✅ **AES-256-GCM Encryption** - All sensitive information is encrypted.
- ✅ **Local Storage** - Data is stored locally only and never uploaded to any server.
- ✅ **Operation Logs** - Complete records for auditing.

## Tech Stack

### Frontend
- **Framework**: Vue 3 + TypeScript
- **UI Components**: Element Plus
- **State Management**: Pinia
- **Build Tool**: Vite
- **Styling**: CSS3 + Element Plus Theme

### Backend
- **Framework**: Tauri 2.x
- **Language**: Rust
- **Encryption**: AES-256-GCM
- **Key Management**: Windows Credential Manager / Keyring
- **Network Requests**: Reqwest
- **Async Runtime**: Tokio

## Installation and Running

### Prerequisites
- Node.js 16+
- Rust 1.70+
- Windows 10/11 (Currently only supports Windows)

### Development Environment

```bash
# Clone the project
git clone [repository-url]
cd windsurf-account-manager

# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Build Release Version

```bash
# Build Windows installer
npm run tauri build
```

The installer will be located in `src-tauri/target/release/bundle/` after the build completes.

## Usage Guide

### 1. First Time Use
1. After launching the app, click the "Add Account" button.
2. Enter your Windsurf account info:
   - Email: Your Windsurf account email.
   - Password: Your account password.
   - Remark Name: An easily identifiable name.
   - Group (Optional): Account grouping.
   - Tags (Optional): Custom tags.
3. Click OK to save the account.

### 2. Credit Reset
1. Click the "Reset Credits" button on an account card.
2. The app will automatically:
   - Log in to get a Token (if needed).
   - Perform a seat update (cycling between 3, 4, and 5 seats).
   - Once successful, credits are reset.
3. Operation results will be shown via notifications.
4. You can enable "Show Detailed Results" in settings to see seat update details.

### 3. Batch Operations
1. Select multiple account cards.
2. Click "Batch Reset Credits" or "Batch Delete" at the top.
3. Confirm to execute.

### 4. Grouping
1. Click the "Grouping" menu in the sidebar.
2. Choose an existing group or add a new one.
3. Select the group when adding or editing accounts.

### 5. View Logs
1. Click "Operation Logs" in the sidebar.
2. Review all operation history.
3. You can clear or export the logs.

## Data Storage
Application data is stored locally:
- **Windows**: `%APPDATA%\com.chao.windsurf-account-manager\accounts.json`

Data structure includes:
- Account info (Encrypted passwords and Tokens)
- Group lists
- System settings
- Operation logs

## Security Note
1. **Password Security**: All passwords are encrypted using AES-256-GCM.
2. **Key Management**: Encryption keys are stored in the system keychain.
3. **Token Refresh**: Tokens are automatically refreshed 5 minutes before expiration.
4. **Local Storage**: All data remains local and is not uploaded to servers.

## License
AGPL-3.0

## Disclaimer
This tool is for educational and personal use only. Please comply with Windsurf's Terms of Service. The author is not responsible for any issues resulting from the use of this tool.
