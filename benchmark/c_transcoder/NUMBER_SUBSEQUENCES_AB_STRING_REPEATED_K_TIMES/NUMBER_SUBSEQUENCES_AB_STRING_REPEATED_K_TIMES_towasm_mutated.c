

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

int f_gold ( char s [], int K ) {
  int n = strlen(s);
  int C, c1 = 0, c2 = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( s [ i ] == 'a' ) c1 ++;
    if ( s [ i ] == 'b' ) {
      c2 ++;
      C += c1;
    }
  }
  return C * K + ( K * ( K - 1 ) / 2 ) * c1 * c2;
}


int main(void) {
	char xv[] = {'a','d'};
	f_gold(xv,29);
}