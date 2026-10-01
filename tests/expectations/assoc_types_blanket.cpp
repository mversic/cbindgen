#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct Wrapper {
  T _0;
};

extern "C" {

uint32_t blanket_identity(uint32_t value);

uint16_t blanket_nested(uint16_t value);

Wrapper<uint8_t> blanket_repeated(Wrapper<uint8_t> value);

uint32_t blanket_nested_const_export(uint32_t value);

}  // extern "C"
