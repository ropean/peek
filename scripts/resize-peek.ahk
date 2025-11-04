#Requires AutoHotkey v2.0

; Find window by executable file name
if WinExist("ahk_exe peek-windows-x64.exe")
{
    ; Resize window, position at (0, 0)
    WinMove(0, 0, 1920, 1080, "ahk_exe peek-windows-x64.exe")
    MsgBox("peek window has been resized")
}
else
{
    MsgBox("Window for peek.exe process not found")
}

ExitApp