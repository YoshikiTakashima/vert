

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

int f_gold ( char s [] ) {
  int result = 0;
  int n = strlen(s);
  for ( int i = 0;
  i < n;
  i ++ ) for ( int j = i;
  j < n;
  j ++ ) if ( s [ i ] == s [ j ] ) result ++;
  return result;
}


int main(void) {
	char xv[] = {'a','b'};
	f_gold(xv);
}