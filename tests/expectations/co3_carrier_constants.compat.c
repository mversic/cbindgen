#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

typedef uint8_t State;
#define STATE_UNKNOWN (uint8_t)0
#define STATE_CHARGING (uint8_t)1
#define STATE_DISCHARGING (uint8_t)42
#define STATE_FULL (uint8_t)43

typedef uint8_t Technology;
#define TECHNOLOGY_UNKNOWN (uint8_t)0
#define TECHNOLOGY_LITHIUM_ION (uint8_t)1
#define TECHNOLOGY_LEAD_ACID (uint8_t)7

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

State battery_get_state(void);

Technology battery_get_technology(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
