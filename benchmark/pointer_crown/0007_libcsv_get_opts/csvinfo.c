#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <stdlib.h>

#define CSV_TAB 9
#define CSV_LF 10
#define CSV_SPACE 10
#define CSV_CR 10

struct csv_parser {
  int pstate;         /* Parser state */
  int quoted;         /* Is the current field a quoted field? */
  size_t spaces;      /* Number of continious spaces after quote or in a non-quoted field */
  unsigned char * entry_buf;   /* Entry buffer */
  size_t entry_pos;   /* Current position in entry_buf (and current size of entry) */
  size_t entry_size;  /* Size of entry buffer */
  int status;         /* Operation status */
  unsigned char options;
  unsigned char quote_char;
  unsigned char delim_char;
  int (*is_space)(unsigned char);
  int (*is_term)(unsigned char);
  size_t blk_size;
  void *(*malloc_func)(size_t);
  void *(*realloc_func)(void *, size_t);
  void (*free_func)(void *);
};

int
csv_get_opts(const struct csv_parser *p)
{
  /* Return the currently set options of parser */
  if (p == NULL)
    return -1;

  return p->options;
}


int main(){
  
}