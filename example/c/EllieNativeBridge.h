#ifndef _ELLIE_NATIVE_BRIDGE_H
#define _ELLIE_NATIVE_BRIDGE_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct IntegerType {
  uintptr_t value;
} IntegerType;

typedef struct StringType {
  uint32_t *value;
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
  struct Types *value;
} ArrayType;

typedef enum Types_Tag {
  TYPES_INTEGER,
  TYPES_STRING,
  TYPES_BOOLEAN,
  TYPES_FLOAT,
  TYPES_DOUBLE,
  TYPES_BYTE,
  TYPES_ARRAY,
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

typedef struct Argument {
  uint32_t *name;
  struct Types argument_type;
} Argument;

typedef struct CallbackError {
  uint32_t *message;
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

typedef struct EllieFunction {
  uint32_t *name;
  struct Argument *arguments;
  struct Types return_type;
  struct ReturnType (*on_call)(struct Argument *arguments);
} EllieFunction;

typedef struct EllieModule {
  uint32_t *name;
  uint32_t *version;
  struct EllieFunction *functions;
} EllieModule;

#endif /* _ELLIE_NATIVE_BRIDGE_H */
