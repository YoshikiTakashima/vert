#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <stdlib.h>

#define CSV_TAB 9
#define CSV_LF 10
#define CSV_SPACE 10
#define CSV_CR 10
#define MEM_BLK_SIZE 128
#define CSV_COMMA  0x2c
#define CSV_QUOTE  0x22

#define ROW_NOT_BEGUN           0
#define FIELD_NOT_BEGUN         1
#define FIELD_BEGUN             2
#define FIELD_MIGHT_HAVE_ENDED  3

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



struct counts {
  long unsigned fields;
  long unsigned rows;
};

void cb1 (void *s, size_t len, void *data) { ((struct counts *)data)->fields++; }
void cb2 (int c, void *data) { ((struct counts *)data)->rows++; }

int
csv_init(struct csv_parser *p, unsigned char options)
{
  /* Initialize a csv_parser object returns 0 on success, -1 on error */
  if (p == NULL)
    return -1;

  p->entry_buf = NULL;
  p->pstate = ROW_NOT_BEGUN;
  p->quoted = 0;
  p->spaces = 0;
  p->entry_pos = 0;
  p->entry_size = 0;
  p->status = 0;
  p->options = options;
  p->quote_char = CSV_QUOTE;
  p->delim_char = CSV_COMMA;
  p->is_space = NULL;
  p->is_term = NULL;
  p->blk_size = MEM_BLK_SIZE;
  p->malloc_func = NULL;
  p->realloc_func = realloc;
  p->free_func = free;

  return 0;
}


int main(){
  
}