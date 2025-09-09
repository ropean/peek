@echo off
setlocal enabledelayedexpansion

echo === C# Hash256 Build Script ===
echo.

rem Navigate to repo root
pushd "%~dp0.."

rem Parse arguments
set CONFIGURATION=Release
set RUNTIME=win-x64
set SELF_CONTAINED=false
set OUTPUT_DIR=publish

:parse_args
if "%~1"=="" goto :build

rem Parse arguments
if /I "%~1"=="debug" set CONFIGURATION=Debug
if /I "%~1"=="release" set CONFIGURATION=Release
if /I "%~1"=="sc" set SELF_CONTAINED=true
if /I "%~1"=="self-contained" set SELF_CONTAINED=true
if /I "%~1"=="fd" set SELF_CONTAINED=false
if /I "%~1"=="framework-dependent" set SELF_CONTAINED=false
shift
goto :parse_args

:build
echo Configuration: %CONFIGURATION%
echo Runtime: %RUNTIME%
echo Self-contained: %SELF_CONTAINED%
echo Output directory: %OUTPUT_DIR%
echo.

rem Clean previous builds
if exist "%OUTPUT_DIR%" rmdir /s /q "%OUTPUT_DIR%"

rem Ensure no running instance is locking files
echo Terminating any running CSharpHash.exe processes...
taskkill /im CSharpHash.exe /f >nul 2>&1

rem Restore dependencies
echo Restoring dependencies...
dotnet restore CSharpHash/CSharpHash.csproj
if errorlevel 1 goto :error

rem Build
echo Building project...
dotnet build CSharpHash/CSharpHash.csproj -c %CONFIGURATION% --no-restore
if errorlevel 1 goto :error

rem Publish
echo Publishing executable...

rem Publish with version handling
@REM echo No VERSION_TAG environment variable found, using default version
@REM echo Running: dotnet publish CSharpHash/CSharpHash.csproj -c %CONFIGURATION% -r %RUNTIME% -p:PublishSingleFile=true --self-contained %SELF_CONTAINED% --output "%OUTPUT_DIR%" --no-restore
dotnet publish CSharpHash/CSharpHash.csproj -c %CONFIGURATION% -r %RUNTIME% -p:PublishSingleFile=true --self-contained %SELF_CONTAINED% --output "%OUTPUT_DIR%" --no-restore

if errorlevel 1 (
    echo Publish failed with error %errorlevel%
    goto :error
)

rem Note: Icon is embedded in the executable via ApplicationIcon in .csproj
echo Icon embedded in executable (no separate file needed).
if errorlevel 1 goto :error

echo.
echo === Build completed successfully! ===
echo Output directory: %CD%\%OUTPUT_DIR%
echo Files created:
dir /b "%OUTPUT_DIR%"
echo.
echo To run: %OUTPUT_DIR%\CSharpHash.exe

popd
exit /b 0

:error
set ERR=%ERRORLEVEL%
echo.
echo === Build failed with error %ERR% ===
popd
exit /b %ERR%


