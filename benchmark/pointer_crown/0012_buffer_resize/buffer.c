#include <string.h>
#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>
#include <ctype.h>
#include <sys/types.h>

typedef struct {
  size_t len;
  char *alloc;
  char *data;
} buffer_t;

#define BUFFER_DEFAULT_SIZE 64
#define nearest_multiple_of(a, b) \
  (((b) + ((a) - 1)) & ~((a) - 1))

/*
 * Resize to hold `n` bytes.
 */

int
buffer_resize(buffer_t *self, size_t n) {
  n = nearest_multiple_of(1024, n);
  self->len = n;
  self->alloc = self->data = realloc(self->alloc, n + 1);
  if (!self->alloc) return -1;
  self->alloc[n] = '\0';
  return 0;
}

/*
 * Append the first `len` bytes from `str` to `self` and
 * return 0 on success, -1 on failure.
 */
int
buffer_append_n(buffer_t *self, const char *str, size_t len) {
  size_t prev = strlen(self->data);
  size_t needed = len + prev;

  // enough space
  if (self->len > needed) {
    strncat(self->data, str, len);
    return 0;
  }

  // resize
  int ret = buffer_resize(self, needed);
  if (-1 == ret) return -1;
  strncat(self->data, str, len);

  return 0;
}

/*
 * Prepend `str` to `self` and return 0 on success, -1 on failure.
 */

int
buffer_prepend(buffer_t *self, char *str) {
  size_t len = strlen(str);
  size_t prev = strlen(self->data);
  size_t needed = len + prev;

  // enough space
  if (self->len > needed) goto move;

  // resize
  int ret = buffer_resize(self, needed);
  if (-1 == ret) return -1;

  // move
  move:
  memmove(self->data + len, self->data, len + 1);
  memcpy(self->data, str, len);

  return 0;
}

int main(){}