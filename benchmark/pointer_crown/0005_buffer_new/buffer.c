
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


/*
 * Allocate a new buffer with `n` bytes.
 */

buffer_t *
buffer_new_with_size(size_t n) {
  buffer_t *self = malloc(sizeof(buffer_t));
  if (!self) return NULL;
  self->len = n;
  self->data = self->alloc = calloc(n + 1, 1);
  return self;
}

void main(){
	
}
