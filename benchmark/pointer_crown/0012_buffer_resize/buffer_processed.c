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



int
buffer_resize(buffer_t *self, size_t n) {
  n = nearest_multiple_of(1024, n);
  self->len = n;
  self->alloc = self->data = realloc(self->alloc, n + 1);
  if (!self->alloc) return -1;
  self->alloc[n] = '\0';
  return 0;
}

int
buffer_append_n(buffer_t *self, const char *str, size_t len) {
  size_t prev = strlen(self->data);
  size_t needed = len + prev;

  if (self->len > needed) {
    strncat(self->data, str, len);
    return 0;
  }

  int ret = buffer_resize(self, needed);
  if (-1 == ret) return -1;
  strncat(self->data, str, len);

  return 0;
}


int
buffer_prepend(buffer_t *self, char *str) {
  size_t len = strlen(str);
  size_t prev = strlen(self->data);
  size_t needed = len + prev;

  if (self->len > needed) goto move;

  int ret = buffer_resize(self, needed);
  if (-1 == ret) return -1;

  move:
  memmove(self->data + len, self->data, len + 1);
  memcpy(self->data, str, len);

  return 0;
}
