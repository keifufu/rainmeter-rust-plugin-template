#include "api_wrapper.h"
#include <stdbool.h>
#include <wchar.h>

wchar_t* RmReadStringWrapper(void* rm, wchar_t* option, wchar_t* defValue, bool replaceMeasures) {
  return RmReadString(rm, option, defValue, replaceMeasures);
}
double RmReadFormulaWrapper(void* rm, wchar_t* option, double defValue) {
  return RmReadFormula(rm, option, defValue);
}
wchar_t* RmReplaceVariablesWrapper(void* rm, wchar_t* str) {
  return RmReplaceVariables(rm, str);
}
wchar_t* RmPathToAbsoluteWrapper(void* rm, wchar_t* relativePath) {
  return RmPathToAbsolute(rm, relativePath);
}
void RmExecuteWrapper(void* skin, wchar_t* command) {
  return RmExecute(skin, command); 
}
void* RmGetWrapper(void* rm, int type) {
  return RmGet(rm, type);
}
void RmLogWrapper(void* rm, int level, wchar_t* message) {
  return RmLog(rm, level, message);
}