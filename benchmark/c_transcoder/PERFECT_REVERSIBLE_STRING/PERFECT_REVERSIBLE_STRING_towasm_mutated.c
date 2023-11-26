

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
  int i = 0, j = strlen(str) - 1;
  while ( i < j ) {
    if ( str [ i ] != str [ j ] ) return 0;
    i ++;
    j --;
  }
  return 1;
}


int main(void) {
	char xv[] = {'a','d'};
	f_gold(xv);
}