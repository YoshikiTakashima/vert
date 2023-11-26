

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
  int n = strlen(str);
  int count = 0;
  char res = str [ 0 ];
  int cur_count = 1;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( i < n - 1 && str [ i ] == str [ i + 1 ] ) cur_count ++;
    else {
      if ( cur_count > count ) {
        count = cur_count;
        res = str [ i ];
      }
      cur_count = 1;
    }
  }
  return res;
}


int main(void) {
	char xv[] = {'a','b'};
	f_gold(xv);
}