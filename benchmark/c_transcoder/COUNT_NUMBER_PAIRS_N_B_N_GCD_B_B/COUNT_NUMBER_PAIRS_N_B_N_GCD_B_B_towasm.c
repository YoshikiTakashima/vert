

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
  int k = n;
  int imin = 1;
  int ans = 0;
  while ( imin <= n ) {
    int imax = n / k;
    ans += k * ( imax - imin + 1 );
    imin = imax + 1;
    k = n / imin;
  }
  return ans;
}


int main(void) {
		f_gold(1);
}