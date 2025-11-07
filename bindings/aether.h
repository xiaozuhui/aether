#ifndef AETHER_H
#define AETHER_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Opaque handle for Aether engine
 */
typedef struct AetherHandle {
  uint8_t _opaque[0];
} AetherHandle;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Create a new Aether engine instance
 *
 * Returns: Pointer to AetherHandle (must be freed with aether_free)
 */
struct AetherHandle *aether_new(void);

/**
 * Create a new Aether engine with all IO permissions enabled
 *
 * Returns: Pointer to AetherHandle (must be freed with aether_free)
 */
struct AetherHandle *aether_new_with_permissions(void);

/**
 * Evaluate Aether code
 *
 * # Parameters
 * - handle: Aether engine handle
 * - code: C string containing Aether code
 * - result: Output parameter for result (must be freed with aether_free_string)
 * - error: Output parameter for error message (must be freed with
 * aether_free_string)
 *
 * # Returns
 * - 0 (Success) if evaluation succeeded
 * - Non-zero error code if evaluation failed
 */
int aether_eval(struct AetherHandle *handle, const char *code, char **result,
                char **error);

/**
 * Get the version string of Aether
 *
 * Returns: C string with version (must NOT be freed)
 */
const char *aether_version(void);

/**
 * Free an Aether engine handle
 */
void aether_free(struct AetherHandle *handle);

/**
 * Free a string allocated by Aether
 */
void aether_free_string(char *s);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* AETHER_H */
