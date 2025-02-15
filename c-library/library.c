#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include "library.h"

person_t* person_new(char* name, uint32_t name_len, uint32_t age) {
    person_t* result = malloc(sizeof(person_t));

    result->name = strndup(name, name_len);
    result->age = age;

    return result;
}

void person_free(person_t* person) {
    free(person->name);
    free(person);
}

char* person_fmt(person_t* person) {
    uint32_t maxlen = strlen(person->name) + 100;
    char* buf = malloc(maxlen);
    memset(buf, 0, maxlen);

    snprintf(buf, maxlen, "%s (age %d)", person->name, person->age);
    
    return buf;
}
