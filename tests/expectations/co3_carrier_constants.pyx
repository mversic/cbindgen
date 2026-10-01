from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef uint8_t State;
  const State STATE_UNKNOWN # = <uint8_t>0
  const State STATE_CHARGING # = <uint8_t>1
  const State STATE_DISCHARGING # = <uint8_t>42
  const State STATE_FULL # = <uint8_t>43

  ctypedef uint8_t Technology;
  const Technology TECHNOLOGY_UNKNOWN # = <uint8_t>0
  const Technology TECHNOLOGY_LITHIUM_ION # = <uint8_t>1
  const Technology TECHNOLOGY_LEAD_ACID # = <uint8_t>7

  State battery_get_state();

  Technology battery_get_technology();
