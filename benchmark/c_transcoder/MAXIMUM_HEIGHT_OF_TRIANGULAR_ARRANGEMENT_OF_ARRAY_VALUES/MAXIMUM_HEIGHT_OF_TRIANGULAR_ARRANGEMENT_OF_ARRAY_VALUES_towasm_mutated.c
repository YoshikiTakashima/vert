

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

int f_gold ( int a [ ], int n ) {
  int result = 1;
  for ( int i = 1;
  i <= n;
  ++ i ) {
    long long y = ( i * ( i + 1 ) ) / 2;
    if ( y < n ) result = i;
    else break;
  }
  return result;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}