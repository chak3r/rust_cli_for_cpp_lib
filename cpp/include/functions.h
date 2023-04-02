#ifndef FUNCTIONS_H
#define FUNCTIONS_H

#ifdef __cplusplus
extern "C" {
#endif

#include "cpp_lib_export.h"
#include "types.h"

CPP_LIB_EXPORT double sum(ACollection a, BCollection b);

CPP_LIB_EXPORT ACollection allocACollection(int size);
CPP_LIB_EXPORT BCollection allocBCollection(int size);

CPP_LIB_EXPORT void freeACollection(ACollection collection);
CPP_LIB_EXPORT void freeBCollection(BCollection collection);

#ifdef __cplusplus
}
#endif

#endif FUNCTIONS_H
