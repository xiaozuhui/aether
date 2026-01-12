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

/**
 * Execution limits configuration
 */
typedef struct AetherLimits {
  int max_steps;
  int max_recursion_depth;
  int max_duration_ms;
} AetherLimits;

/**
 * Cache statistics
 */
typedef struct AetherCacheStats {
  int hits;
  int misses;
  int size;
} AetherCacheStats;

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
 * - error: Output parameter for error message (must be freed with aether_free_string)
 *
 * # Returns
 * - 0 (Success) if evaluation succeeded
 * - Non-zero error code if evaluation failed
 */
int aether_eval(struct AetherHandle *handle, const char *code, char **result, char **error);

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

/**
 * Set a global variable from host application
 *
 * # Parameters
 * - handle: Aether engine handle
 * - name: Variable name
 * - value_json: Variable value as JSON string
 *
 * # Returns
 * - 0 (Success) if variable was set
 * - Non-zero error code if failed
 *
 * # Safety
 * - `handle` must be a valid pointer to an AetherHandle created by `aether_new` or `aether_new_with_permissions`
 * - `name` must be a valid pointer to a null-terminated C string
 * - `value_json` must be a valid pointer to a null-terminated C string
 */
int aether_set_global(struct AetherHandle *handle,
                      const char *name,
                      const char *value_json);

/**
 * Get a variable's value as JSON
 *
 * # Parameters
 * - handle: Aether engine handle
 * - name: Variable name
 * - value_json: Output parameter (must be freed with aether_free_string)
 *
 * # Returns
 * - 0 (Success) if variable was found
 * - VariableNotFound (6) if variable doesn't exist
 * - Non-zero error code for other failures
 *
 * # Safety
 * - `handle` must be a valid pointer to an AetherHandle created by `aether_new` or `aether_new_with_permissions`
 * - `name` must be a valid pointer to a null-terminated C string
 * - `value_json` must be a valid pointer to a `*mut c_char` that will be set to point to the result
 */
int aether_get_global(struct AetherHandle *handle,
                      const char *name,
                      char **value_json);

/**
 * Reset the runtime environment (clears all variables)
 *
 * # Parameters
 * - handle: Aether engine handle
 */
void aether_reset_env(struct AetherHandle *handle);

/**
 * Get all trace entries as JSON array
 *
 * # Parameters
 * - handle: Aether engine handle
 * - trace_json: Output parameter (must be freed with aether_free_string)
 *
 * # Returns
 * - 0 (Success) if trace was retrieved
 * - Non-zero error code if failed
 *
 * # Safety
 * - `handle` must be a valid pointer to an AetherHandle created by `aether_new` or `aether_new_with_permissions`
 * - `trace_json` must be a valid pointer to a `*mut c_char` that will be set to point to the result
 */
int aether_take_trace(struct AetherHandle *handle,
                      char **trace_json);

/**
 * Clear the trace buffer
 *
 * # Parameters
 * - handle: Aether engine handle
 */
void aether_clear_trace(struct AetherHandle *handle);

/**
 * Get structured trace entries as JSON
 *
 * # Parameters
 * - handle: Aether engine handle
 * - trace_json: Output parameter (must be freed with aether_free_string)
 *
 * # Returns
 * - 0 (Success) if trace was retrieved
 * - Non-zero error code if failed
 *
 * # Safety
 * - `handle` must be a valid pointer to an AetherHandle created by `aether_new` or `aether_new_with_permissions`
 * - `trace_json` must be a valid pointer to a `*mut c_char` that will be set to point to the result
 */
int aether_trace_records(struct AetherHandle *handle,
                         char **trace_json);

/**
 * Get trace statistics as JSON
 *
 * # Parameters
 * - handle: Aether engine handle
 * - stats_json: Output parameter (must be freed with aether_free_string)
 *
 * # Returns
 * - 0 (Success) if stats were retrieved
 * - Non-zero error code if failed
 *
 * # Safety
 * - `handle` must be a valid pointer to an AetherHandle created by `aether_new` or `aether_new_with_permissions`
 * - `stats_json` must be a valid pointer to a `*mut c_char` that will be set to point to the result
 */
int aether_trace_stats(struct AetherHandle *handle,
                       char **stats_json);

/**
 * Set execution limits
 *
 * # Parameters
 * - handle: Aether engine handle
 * - limits: Limits configuration
 *
 * # Safety
 * - `handle` must be a valid pointer to an AetherHandle created by `aether_new` or `aether_new_with_permissions`
 * - `limits` must be a valid pointer to an AetherLimits struct
 */
void aether_set_limits(struct AetherHandle *handle,
                       const struct AetherLimits *limits);

/**
 * Get current execution limits
 *
 * # Parameters
 * - handle: Aether engine handle
 * - limits: Output parameter
 *
 * # Safety
 * - `handle` must be a valid pointer to an AetherHandle created by `aether_new` or `aether_new_with_permissions`
 * - `limits` must be a valid pointer to an AetherLimits struct that will be filled with the current limits
 */
void aether_get_limits(struct AetherHandle *handle,
                       struct AetherLimits *limits);

/**
 * Clear the AST cache
 *
 * # Parameters
 * - handle: Aether engine handle
 */
void aether_clear_cache(struct AetherHandle *handle);

/**
 * Get cache statistics
 *
 * # Parameters
 * - handle: Aether engine handle
 * - stats: Output parameter
 *
 * # Safety
 * - `handle` must be a valid pointer to an AetherHandle created by `aether_new` or `aether_new_with_permissions`
 * - `stats` must be a valid pointer to an AetherCacheStats struct that will be filled with the statistics
 */
void aether_cache_stats(struct AetherHandle *handle,
                        struct AetherCacheStats *stats);

/**
 * Set optimization options
 *
 * # Parameters
 * - handle: Aether engine handle
 * - constant_folding: Enable constant folding (1 = yes, 0 = no)
 * - dead_code_elimination: Enable dead code elimination (1 = yes, 0 = no)
 * - tail_recursion: Enable tail recursion optimization (1 = yes, 0 = no)
 */
void aether_set_optimization(struct AetherHandle *handle,
                             int constant_folding,
                             int dead_code_elimination,
                             int tail_recursion);

extern void log(const str *s);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* AETHER_H */
