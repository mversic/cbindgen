from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Wrapper_u8:
    uint8_t _0;

  uint32_t blanket_identity(uint32_t value);

  uint16_t blanket_nested(uint16_t value);

  Wrapper_u8 blanket_repeated(Wrapper_u8 value);

  uint32_t blanket_nested_const_export(uint32_t value);
