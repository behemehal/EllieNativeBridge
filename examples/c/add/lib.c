#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "ellie_native_bridge.h"

static FunctionAnswer add(const FunctionCallParameter *args) {
    if (args == NULL) {
        return (FunctionAnswer) {
            .tag = FUNCTION_ANSWER_RUNTIME_ERROR,
            .runtime_error = "No arguments provided"
        };
    }

    if (args[0].data.tag != ELLIE_DATA_INTEGER || args[1].data.tag != ELLIE_DATA_INTEGER) {
        return (FunctionAnswer) {
            .tag = FUNCTION_ANSWER_RUNTIME_ERROR,
            .runtime_error = "Expected integer arguments"
        };
    }

    intptr_t a = args[0].data.integer.as_isize;
    intptr_t b = args[1].data.integer.as_isize;



    intptr_t result = a + b;

    EllieData data = {
        .tag = ELLIE_DATA_INTEGER,
        .integer = {
            .as_isize = result
        }
    };

    return (FunctionAnswer) {
        .tag = FUNCTION_ANSWER_OK,
        .ok = data
    };
}

EllieModule load_module() {

    EllieFunction functions[] = {
        {
            .name = "add",
            .on_call = add
        },
        {
            NULL, NULL
        }
    };

    EllieModule module = {
        .name = "RustMath",
        .functions = functions
    };

    return module;
}