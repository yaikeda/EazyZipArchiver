# EazyZipArchiver

EazyZipArchiver is a lightweight and simple file compression tool built with .NET.  
It allows users to zip files and folders via a drag-and-drop GUI interface, with a focus on minimal setup and ease of use.

---

## ✨ Features

- 📦 Drag-and-drop interface for quick file archiving
- 📁 Support for both single files and directories
- 🔧 Simple configuration with `.config` file support
- 🪶 Minimal dependencies (pure .NET-based)
- 🖥️ Compact and clean UI

---

## 🛠 Requirements

- .NET Framework (version depends on your project setting — typically 4.5 or later)
- Visual Studio (for development/building)

---

## 🚀 How to Use

1. **Launch the application**
2. **Drag and drop** files or folders into the window
3. A `.zip` archive will be created in the same directory

You can also configure default behavior via the accompanying `.config` file (see below).

---

## ⚙ Configuration

The app supports a simple config system to customize behavior.

Example:

```xml
<appSettings>
  <add key="DefaultCompressionLevel" value="Fastest"/>
  <add key="OverwriteExisting" value="true"/>
</appSettings>
