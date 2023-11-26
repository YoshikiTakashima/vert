

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

float f_gold ( double a, double b ) {
  double mod;
  if ( a < 0 ) mod = - a;
  else mod = a;
  if ( b < 0 ) b = - b;
  while ( mod >= b ) mod = mod - b;
  if ( a < 0 ) return - mod;
  return mod;
}


int main(void) {
		f_gold(1,2);
}