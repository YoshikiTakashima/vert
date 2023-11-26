

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

int f_gold ( int notes [ ], int n ) {
  int fiveCount = 0;
  int tenCount = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( notes [ i ] == 5 ) fiveCount ++;
    else if ( notes [ i ] == 10 ) {
      if ( fiveCount > 0 ) {
        fiveCount --;
        tenCount ++;
      }
      else return 0;
    }
    else {
      if ( fiveCount > 0 && tenCount > 0 ) {
        fiveCount --;
        tenCount --;
      }
      else if ( fiveCount >= 3 ) {
        fiveCount -= 3;
      }
      else return 0;
    }
  }
  return 1;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}