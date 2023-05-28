#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "EllieNativeBridge.h"

ReturnType println(Argument *args)
{
    StringType *stringArg = (StringType *)&args[0].argument_type;

    printf("%s\n", (char *)stringArg->value);

    ReturnType result;

    result.tag = RETURN_TYPE_VALUE;
    result.value.tag = TYPES_BOOLEAN;
    result.value.boolean.value = false;

    return result;
}

EllieModule getLib()
{
    Argument printlnArgs[] = {
        {
            .name = (uint32_t *)"string",
            .argument_type = {
                .tag = TYPES_STRING,
                .string = {
                    .value = NULL
                }
            }
        }
    };

    EllieFunction printlnFunction;
    printlnFunction.name = (uint32_t *)"println";
    printlnFunction.arguments = printlnArgs;
    printlnFunction.return_type.tag = TYPES_BOOLEAN;
    printlnFunction.return_type.boolean.value = false;
    printlnFunction.on_call = &println;

    EllieModule module = {
        .name = (uint32_t *)"C",
        .version = (uint32_t *)"0.0.1",
        .functions = &printlnFunction,
    };

    return module;
}
