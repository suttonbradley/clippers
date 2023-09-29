# Flow (Windows API Planning)

- ~~`SetClipboardViewer` to add this windows to the [_clipboard viewer chain_](https://learn.microsoft.com/en-us/windows/win32/dataxchg/about-the-clipboard#clipboard-viewers)~~
- ^ instead, create and use a [Clipboard Format Listener](https://learn.microsoft.com/en-us/windows/win32/dataxchg/using-the-clipboard#creating-a-clipboard-format-listener)

# Misc
- To disable Win+V, set reg key `Computer\HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Clipboard\IsCloudAndHistoryFeatureAvailable` to 0
