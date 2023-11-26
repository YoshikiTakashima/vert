

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



unsigned int f_gold ( unsigned int x ) {
  unsigned int even_bits = x & 0xAAAAAAAA;
  unsigned int odd_bits = x & 0x55555555;
  even_bits >>= 1;
  odd_bits <<= 1;
  return ( even_bits | odd_bits );
}


