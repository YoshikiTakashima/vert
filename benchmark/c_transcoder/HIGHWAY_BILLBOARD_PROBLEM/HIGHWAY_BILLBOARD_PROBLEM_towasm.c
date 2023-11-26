

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

int f_gold ( int m, int x [ ], int revenue [ ], int n, int t ) {
  int maxRev [ m + 1 ];
  memset ( maxRev, 0, sizeof ( maxRev ) );
  int nxtbb = 0;
  for ( int i = 1;
  i <= m;
  i ++ ) {
    if ( nxtbb < n ) {
      if ( x [ nxtbb ] != i ) maxRev [ i ] = maxRev [ i - 1 ];
      else {
        if ( i <= t ) maxRev [ i ] = max ( maxRev [ i - 1 ], revenue [ nxtbb ] );
        else maxRev [ i ] = max ( maxRev [ i - t - 1 ] + revenue [ nxtbb ], maxRev [ i - 1 ] );
        nxtbb ++;
      }
    }
    else maxRev [ i ] = maxRev [ i - 1 ];
  }
  return maxRev [ m ];
}


int main(void) {
		int yq[] = {11,12};
	int qe[] = {11,12};
	f_gold(1,yq,qe,4,5);
}