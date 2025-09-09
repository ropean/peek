@echo off
setlocal

pushd "%~dp0.."

echo Cleaning bin/obj...
if exist CSharpHash\bin rmdir /s /q CSharpHash\bin
if exist CSharpHash\obj rmdir /s /q CSharpHash\obj

echo Cleaning publish directory...
if exist publish rmdir /s /q publish

echo Running dotnet clean...
dotnet clean CSharpHash >nul 2>&1

echo Removing any remaining build artifacts...
for /r . %%d in (bin obj) do @if exist "%%d" rmdir /s /q "%%d" 2>nul

echo Done.
popd
exit /b 0