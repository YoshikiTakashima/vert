

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
  if ( n == 0 ) return 0;
  if ( n == 1 ) return ( ( str [ 0 ] - '0' ) % 4 == 0 );
  int last = str [ n - 1 ] - '0';
  int second_last = str [ n - 2 ] - '0';
  return ( ( second_last * 10 + last ) % 4 == 0 );
}


int main(void) {
	char xv[] = {'a','d'};
	f_gold(xv);
}