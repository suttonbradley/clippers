# Flow (Windows API Planning)

- ~~`SetClipboardViewer` to add this windows to the [_clipboard viewer chain_](https://learn.microsoft.com/en-us/windows/win32/dataxchg/about-the-clipboard#clipboard-viewers)~~
- ^ instead, create and use a [Clipboard Format Listener](https://learn.microsoft.com/en-us/windows/win32/dataxchg/using-the-clipboard#creating-a-clipboard-format-listener)

# TODO
- [ ] Add proper logging (to file? since this will be a daemon)
- [ ] Make sure only one instance of the app is running
    - For debug, should be able to set more verbose log level and run, but should popup error if the version run as daemon is _not_ already running.
- [ ] Setup binary that sets the `Computer\HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Clipboard\IsCloudAndHistoryFeatureAvailable` key to 0

# Misc
- To disable Win+V, set reg key `Computer\HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Clipboard\IsCloudAndHistoryFeatureAvailable` to 0
