@echo off

set PLUGIN_NAME=rustplugin
set PLUGIN_DLL=%PLUGIN_NAME%.dll
set PLUGIN_DIRECTORY=%APPDATA%\Rainmeter\Plugins

rem Build the plugin
cargo build --release

rem Kill Rainmeter
taskkill /IM Rainmeter.exe /F

rem Copy the DLL to the Rainmeter plugins directory
xcopy /Y /I /F "target\release\%PLUGIN_DLL%" "%PLUGIN_DIRECTORY%\%PLUGIN_DLL%*"

rem Start Rainmeter
start "" "C:\Program Files\Rainmeter\Rainmeter.exe"
