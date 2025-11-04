#Requires AutoHotkey v2.0

; Find window by executable file name
if WinExist("ahk_exe peek.exe")
{
    ; Resize window, position at (0, 0)
    WinMove(0, 0, 1200, 675, "ahk_exe peek.exe")
    MsgBox("peek window has been resized")
}
else
{
    MsgBox("Window for peek.exe process not found")
}

ExitApp