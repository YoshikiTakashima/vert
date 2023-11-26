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

#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int N, int insert, int remove, int copy ) {
  if ( N == 0 ) return 0;
  if ( N == 1 ) return insert;
  int dp [ N + 1 ];
  memset ( dp, 0, sizeof ( dp ) );
  for ( int i = 1;
  i <= N;
  i ++ ) {
    if ( i % 2 == 0 ) dp [ i ] = min ( dp [ i - 1 ] + insert, dp [ i / 2 ] + copy );
    else dp [ i ] = min ( dp [ i - 1 ] + insert, dp [ ( i + 1 ) / 2 ] + copy + remove );
  }
  return dp [ N ];
}


int main(void) {
		f_gold(1,2,3,4);
}