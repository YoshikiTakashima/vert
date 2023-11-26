

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

int f_gold ( char str [] ) {
  int n = strlen(str);
  int i;
  for ( i = 0;
  i < n;
  i ++ ) if ( str [ i ] != 'a' ) break;
  if ( i * 2 != n ) return 0;
  int j;
  for ( j = i;
  j < n;
  j ++ ) if ( str [ j ] != 'b' ) return 0;
  return 1;
}


int main(void) {
	char xv[] = {'a','d'};
	f_gold(xv);
}