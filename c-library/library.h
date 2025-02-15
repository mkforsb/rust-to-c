#ifndef HAVE_LIBRARY_H
#define HAVE_LIBRARY_H

#include <stdint.h>

typedef struct {
    char* name;
    uint32_t age;
} person_t;

person_t* person_new(char* name, uint32_t name_len, uint32_t age);
void person_free(person_t* person);
char* person_fmt(person_t* person);

#else
#endif

