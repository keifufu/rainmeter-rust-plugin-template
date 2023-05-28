@echo off

:: ============= ::
:: Configuration
:: ============= ::

:: If plugin should be copied to the plugin folder after building
set COPY_PLUGIN_AFTER_BUILD="false"
:: The architecture to copy (x64 or x86)
set COPY_ARCHITECTURE="x64"
:: The rainmeter plugin directory path
set PLUGIN_DIRECTORY_PATH="%APPDATA%\Rainmeter\Plugins"
:: The rainmeter executable path
set RAINMETER_EXE_PATH="C:\Program Files\Rainmeter\Rainmeter.exe"

:: ============= ::
:: Configuration
:: ============= ::

rustup target list | findstr /C:"x86_64-pc-windows-msvc (installed)" > nul 2>&1
if errorLevel 1 (
  echo Installing x86_64-pc-windows-msvc target...
  rustup target add x86_64-pc-windows-msvc
)

:: reset error level
cmd /c "exit /b 0"

rustup target list | findstr /C:"i686-pc-windows-msvc (installed)" > nul 2>&1
if errorLevel 1 (
  echo Installing i686-pc-windows-msvc target...
  rustup target add i686-pc-windows-msvc
)

:: reset error level
cmd /c "exit /b 0"

echo Building 'x64'
cargo build --release --target x86_64-pc-windows-msvc

echo Building 'x86'
cargo build --release --target i686-pc-windows-msvc

echo Copying plugins to /dist
:: xcopy creates folders for us
for %%F in ("%~dp0\target\x86_64-pc-windows-msvc\release\*.dll") do (
  xcopy /Y "%%F" "%~dp0\dist\x64\%%~nxF*" > nul 2>&1
)
for %%F in ("%~dp0\target\i686-pc-windows-msvc\release\*.dll") do (
  xcopy /Y "%%F" "%~dp0\dist\x86\%%~nxF*" > nul 2>&1
)

if %COPY_PLUGIN_AFTER_BUILD% == "true" (
  taskkill /f /im rainmeter.exe > nul 2>&1
  echo Copying %COPY_ARCHITECTURE% plugin to plugin folder
  for %%F in ("%~dp0\dist\%COPY_ARCHITECTURE%\*.dll") do (
    xcopy /Y "%%F" "%PLUGIN_DIRECTORY_PATH%\%%~nxF*" > nul 2>&1
  )
  start "" %RAINMETER_EXE_PATH%
)

echo Done