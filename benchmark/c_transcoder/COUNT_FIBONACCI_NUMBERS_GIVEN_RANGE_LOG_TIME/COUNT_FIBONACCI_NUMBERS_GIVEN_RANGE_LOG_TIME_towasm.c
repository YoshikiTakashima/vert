

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

int f_gold ( int low, int high ) {
  int f1 = 0, f2 = 1, f3 = 1;
  int result = 0;
  while ( f1 <= high ) {
    if ( f1 >= low ) result ++;
    f1 = f2;
    f2 = f3;
    f3 = f1 + f2;
  }
  return result;
}


int main(void) {
		f_gold(1,2);
}