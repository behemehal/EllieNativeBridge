#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "EllieNativeBridge.h"

ReturnType println(ThreadInfo _, Types *args)
{

    if (sizeof(args) != 1)
    {
        ReturnType wrongSignature;

        wrongSignature.tag = RETURN_TYPE_ERROR;
        wrongSignature.error.message = "Signature mismatch expected 1 argument(s)";
        wrongSignature.error.code = 0;
        return wrongSignature;
    }

    if (args[0].tag != TYPES_STRING)
    {
        ReturnType wrongType;

        wrongType.tag = RETURN_TYPE_ERROR;
        wrongType.error.message = "Argument 1 must be a string";
        wrongType.error.code = 0;
        return wrongType;
    }

    StringType *stringArg = (StringType *)&args[0].string;
    printf("%s\n", (char *)stringArg->value);

    ReturnType result;
    result.tag = RETURN_TYPE_VOID;
    return result;
}

ReturnType print(ThreadInfo _, Types *args)
{

    if (sizeof(args) != 1)
    {
        ReturnType wrongSignature;

        wrongSignature.tag = RETURN_TYPE_ERROR;
        wrongSignature.error.message = "Signature mismatch expected 1 argument(s)";
        wrongSignature.error.code = 0;
        return wrongSignature;
    }

    if (args[0].tag != TYPES_STRING)
    {
        ReturnType wrongType;

        wrongType.tag = RETURN_TYPE_ERROR;
        wrongType.error.message = "Argument 1 must be a string";
        wrongType.error.code = 0;
        return wrongType;
    }

    StringType *stringArg = (StringType *)&args[0].string;
    printf("%s\n", (char *)stringArg->value);

    ReturnType result;
    result.tag = RETURN_TYPE_VOID;
    return result;
}

const EllieFunction example_functions[] = {
    {"println", &println},
    {"print", &print},
};

const EllieModule module = {"example_module", example_functions};

void onInit()
{
    printf("C: onInit() on '%s' with '%lu'\n", module.name, sizeof(module.functions));
}
