#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Wrapper_u8 {
  uint8_t _0;
} Wrapper_u8;

uint32_t blanket_identity(uint32_t value);

uint16_t blanket_nested(uint16_t value);

struct Wrapper_u8 blanket_repeated(struct Wrapper_u8 value);

uint32_t blanket_nested_const_export(uint32_t value);
