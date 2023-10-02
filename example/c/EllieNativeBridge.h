#ifndef _ELLIE_NATIVE_BRIDGE_H
#define _ELLIE_NATIVE_BRIDGE_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct IntegerType {
  intptr_t value;
} IntegerType;

typedef struct StringType {
  const char *value;
} StringType;

typedef struct BooleanType {
  bool value;
} BooleanType;

typedef struct FloatType {
  double value;
} FloatType;

typedef struct DoubleType {
  float value;
} DoubleType;

typedef struct ByteType {
  uint8_t value;
} ByteType;

typedef struct ArrayType {
  const struct Types *value;
} ArrayType;

typedef enum Types_Tag {
  TYPES_INTEGER,
  TYPES_STRING,
  TYPES_BOOLEAN,
  TYPES_FLOAT,
  TYPES_DOUBLE,
  TYPES_BYTE,
  TYPES_ARRAY,
  TYPES_NULL,
} Types_Tag;

typedef struct Types {
  Types_Tag tag;
  union {
    struct {
      struct IntegerType integer;
    };
    struct {
      struct StringType string;
    };
    struct {
      struct BooleanType boolean;
    };
    struct {
      struct FloatType float_;
    };
    struct {
      struct DoubleType double_;
    };
    struct {
      struct ByteType byte;
    };
    struct {
      struct ArrayType array;
    };
  };
} Types;

typedef struct CallbackError {
  const char *message;
  uintptr_t code;
} CallbackError;

typedef enum ReturnType_Tag {
  RETURN_TYPE_VALUE,
  RETURN_TYPE_VOID,
  RETURN_TYPE_ERROR,
} ReturnType_Tag;

typedef struct ReturnType {
  ReturnType_Tag tag;
  union {
    struct {
      struct Types value;
    };
    struct {
      struct CallbackError error;
    };
  };
} ReturnType;

typedef struct ThreadInfo {
  uintptr_t id;
  uintptr_t stack_id;
  uintptr_t stack_caller;
} ThreadInfo;

typedef struct ReturnType (*FunctionElementCallback)(struct ThreadInfo thread_info,
                                                     struct Types *params);

typedef struct EllieFunction {
  const char *name;
  FunctionElementCallback on_call;
} EllieFunction;

typedef struct EllieModule {
  const char *name;
  const struct EllieFunction *functions;
} EllieModule;

#endif /* _ELLIE_NATIVE_BRIDGE_H */
