Because of x86 c++ naming conventions, rust is having a stroke calling these functions directly, whereas with x86_64 it works fine.

This is a C wrapper around the x86 rainmeter api to make rust able to call it.

### Building
If you for some reason need to build this, install MinGW-w64 and add it to your system environment path, then run build.bat