#ifndef TYPES_H
#define TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

#include "cpp_lib_export.h"
typedef struct CPP_LIB_EXPORT{
  double first;
  double second;
} A;

typedef struct CPP_LIB_EXPORT {
  A *data;
  int size;
} ACollection;

typedef struct CPP_LIB_EXPORT {
  double first;
  double second;
} B;

typedef struct CPP_LIB_EXPORT{
  B *data;
  int size;
} BCollection;

#ifdef __cplusplus
}
#endif

#endif TYPES_H
