#include <stdbool.h>
#include <wchar.h>
#ifndef WRAPPER_H
#define WRAPPER_H

wchar_t* __stdcall RmReadString(void* rm, wchar_t* option, wchar_t* defValue, bool replaceMeasures);
double __stdcall RmReadFormula(void* rm, wchar_t* option, double defValue);
wchar_t* __stdcall RmReplaceVariables(void* rm, wchar_t* str);
wchar_t* __stdcall RmPathToAbsolute(void* rm, wchar_t* relativePath);
void __stdcall RmExecute(void* skin, wchar_t* command);
void* __stdcall RmGet(void* rm, int type);
void __stdcall RmLog(void* rm, int level, wchar_t* message);

__declspec(dllexport) wchar_t* __cdecl RmReadStringWrapper(void* rm, wchar_t* option, wchar_t* defValue, bool replaceMeasures);
__declspec(dllexport) double __cdecl RmReadFormulaWrapper(void* rm, wchar_t* option, double defValue);
__declspec(dllexport) wchar_t* __cdecl RmReplaceVariablesWrapper(void* rm, wchar_t* str);
__declspec(dllexport) wchar_t* __cdecl RmPathToAbsoluteWrapper(void* rm, wchar_t* relativePath);
__declspec(dllexport) void __cdecl RmExecuteWrapper(void* skin, wchar_t* command);
__declspec(dllexport) void* __cdecl RmGetWrapper(void* rm, int type);
__declspec(dllexport) void __cdecl RmLogWrapper(void* rm, int level, wchar_t* message);

#endif