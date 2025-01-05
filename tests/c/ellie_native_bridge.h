#ifndef _ELLIE_NATIVE_BRIDGE_H
#define _ELLIE_NATIVE_BRIDGE_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct EllieInteger {
  intptr_t as_isize;
  uintptr_t as_usize;
} EllieInteger;

typedef enum EllieData_Tag {
  ELLIE_DATA_INTEGER,
  ELLIE_DATA_FLOAT,
  ELLIE_DATA_DOUBLE,
  ELLIE_DATA_BYTE,
  ELLIE_DATA_BOOL,
  ELLIE_DATA_STRING,
  ELLIE_DATA_CHAR,
  ELLIE_DATA_VOID,
  ELLIE_DATA_NULL,
  ELLIE_DATA_ARRAY,
  ELLIE_DATA_CLASS,
} EllieData_Tag;

typedef struct EllieData {
  EllieData_Tag tag;
  union {
    struct {
      struct EllieInteger integer;
    };
    struct {
      float float_;
    };
    struct {
      double double_;
    };
    struct {
      uint8_t byte;
    };
    struct {
      bool bool_;
    };
    struct {
      const char *string;
    };
    struct {
      char char_;
    };
    struct {
      const struct EllieData *array;
    };
    struct {
      const struct EllieData *class_;
    };
  };
} EllieData;

typedef enum FunctionAnswer_Tag {
  FUNCTION_ANSWER_RUNTIME_ERROR,
  FUNCTION_ANSWER_OK,
} FunctionAnswer_Tag;

typedef struct FunctionAnswer {
  FunctionAnswer_Tag tag;
  union {
    struct {
      const char *runtime_error;
    };
    struct {
      struct EllieData ok;
    };
  };
} FunctionAnswer;

typedef struct FunctionCallParameter {
  struct EllieData data;
  uintptr_t memory_location;
} FunctionCallParameter;

typedef struct EllieFunction {
  const char *name;
  struct FunctionAnswer (*on_call)(const struct FunctionCallParameter *arguments);
} EllieFunction;

typedef struct EllieModule {
  const char *name;
  const char *version;
  const struct EllieFunction *functions;
} EllieModule;

#endif  /* _ELLIE_NATIVE_BRIDGE_H */
