

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
  sort ( a, a + n );
  int num1 = 0;
  int num2 = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( i % 2 == 0 ) num1 = num1 * 10 + a [ i ];
    else num2 = num2 * 10 + a [ i ];
  }
  return num2 + num1;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}