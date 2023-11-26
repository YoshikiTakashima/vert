

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int min(int x, int y) { return (x < y)? x: y; }
int max(int x, int y) { return (x > y)? x: y; }
int cmpfunc (const void * a, const void * b) {return ( *(int*)a - *(int*)b );}
int len (int arr [ ]) {return ((int) (sizeof (arr) / sizeof (arr)[0]));}
void sort (int arr [ ], int n) {qsort (arr, n, sizeof(int), cmpfunc);}

int f_gold ( int n ) {
  if ( n <= 1 ) return n;
  int a = 0, b = 1, c = 1;
  int res = 1;
  while ( c < n ) {
    c = a + b;
    res ++;
    a = b;
    b = c;
  }
  return res;
}


int main(void) {
		f_gold(1);
}