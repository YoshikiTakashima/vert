

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

char f_gold ( char str [] ) {
  int len = strlen(str);
  int count = 0;
  char res = str [ 0 ];
  for ( int i = 0;
  i < len;
  i ++ ) {
    int cur_count = 1;
    for ( int j = i + 1;
    j < len;
    j ++ ) {
      if ( str [ i ] != str [ j ] ) break;
      cur_count ++;
    }
    if ( cur_count > count ) {
      count = cur_count;
      res = str [ i ];
    }
  }
  return res;
}


int main(void) {
	char xv[] = {'a','d'};
	f_gold(xv);
}