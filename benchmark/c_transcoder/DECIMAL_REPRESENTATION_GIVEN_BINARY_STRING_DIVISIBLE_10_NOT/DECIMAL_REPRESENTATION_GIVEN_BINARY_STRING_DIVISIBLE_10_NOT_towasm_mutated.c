

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

int f_gold ( char bin [] ) {
  int n = len(bin);
  if ( bin [ n - 1 ] == '1' ) return 0;
  int sum = 0;
  for ( int i = n - 2;
  i >= 0;
  i -- ) {
    if ( bin [ i ] == '1' ) {
      int posFromRight = n - i - 1;
      if ( posFromRight % 4 == 1 ) sum = sum + 2;
      else if ( posFromRight % 4 == 2 ) sum = sum + 4;
      else if ( posFromRight % 4 == 3 ) sum = sum + 8;
      else if ( posFromRight % 4 == 0 ) sum = sum + 6;
    }
  }
  if ( sum % 10 == 0 ) return 1;
  return 0;
}


int main(void) {
	char xv[] = {'a','d'};
	f_gold(xv);
}