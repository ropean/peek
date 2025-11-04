@echo off
setlocal enabledelayedexpansion
echo Building peek ...
cd /d "%~dp0.."
cargo build --release
if %ERRORLEVEL% EQU 0 (
    echo Build successful!
    
    REM Show file size before compression
    if exist target\release\peek.exe (
        for %%A in (target\release\peek.exe) do set SIZE_BYTES=%%~zA
        set /a SIZE_MB=!SIZE_BYTES! / 1048576
        set /a SIZE_REMAINDER=!SIZE_BYTES! * 100 / 1048576
        set /a SIZE_DECIMAL=!SIZE_REMAINDER! - !SIZE_MB! * 100
        echo File size before compression: !SIZE_MB!.!SIZE_DECIMAL! MB
    )
    
    REM Check if UPX is available
    where upx >nul 2>&1
    if %ERRORLEVEL% EQU 0 (
        echo Compressing with UPX...
        echo.
        upx --best --lzma target\release\peek.exe
        if %ERRORLEVEL% EQU 0 (
            echo.
            echo Compression successful!
        ) else (
            REM Check if already compressed
            echo Checking if file is already compressed...
            upx -t target\release\peek.exe
            if %ERRORLEVEL% EQU 0 (
                echo File already compressed with UPX
            ) else (
                echo Compression failed, but build is OK
            )
        )
    ) else (
        echo.
        echo Note: UPX not found. Install it to reduce executable size:
        echo   - Windows: scoop install upx  or  choco install upx
        echo   - Download: https://github.com/upx/upx/releases
        echo.
    )
    
    REM Show final file size
    if exist target\release\peek.exe (
        for %%A in (target\release\peek.exe) do set SIZE_BYTES=%%~zA
        set /a SIZE_MB=!SIZE_BYTES! / 1048576
        set /a SIZE_REMAINDER=!SIZE_BYTES! * 100 / 1048576
        set /a SIZE_DECIMAL=!SIZE_REMAINDER! - !SIZE_MB! * 100
        echo.
        echo ===================================
        echo Final executable size: !SIZE_MB!.!SIZE_DECIMAL! MB
        echo ===================================
        echo Executable available at: target\release\peek.exe
    )
) else (
    echo Build failed!
)
