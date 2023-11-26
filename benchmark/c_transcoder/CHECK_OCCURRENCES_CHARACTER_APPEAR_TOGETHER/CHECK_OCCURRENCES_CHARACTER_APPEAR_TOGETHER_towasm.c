

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

int f_gold ( char s [], char c ) {
  bool oneSeen = 0;
  int i = 0, n = strlen(s);
  while ( i < n ) {
    if ( s [ i ] == c ) {
      if ( oneSeen == 1 ) return 0;
      while ( i < n && s [ i ] == c ) i ++;
      oneSeen = 1;
    }
    else i ++;
  }
  return 1;
}


int main(void) {
	char xv[] = {'a','b'};
	f_gold(xv,b);
}