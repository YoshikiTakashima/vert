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
int f_gold ( int arr [ ], int n ) {
  if ( n == 1 ) return arr [ 0 ];
  int dec [ n ];
  memset ( dec, 0, sizeof ( dec ) );
  int inc [ n ];
  memset ( inc, 0, sizeof ( inc ) );
  dec [ 0 ] = inc [ 0 ] = arr [ 0 ];
  int flag = 0;
  for ( int i = 1;
  i < n;
  i ++ ) {
    for ( int j = 0;
    j < i;
    j ++ ) {
      if ( arr [ j ] > arr [ i ] ) {
        dec [ i ] = max ( dec [ i ], inc [ j ] + arr [ i ] );
        flag = 1;
      }
      else if ( arr [ j ] < arr [ i ] && flag == 1 ) inc [ i ] = max ( inc [ i ], dec [ j ] + arr [ i ] );
    }
  }
  int result = INT_MIN;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( result < inc [ i ] ) result = inc [ i ];
    if ( result < dec [ i ] ) result = dec [ i ];
  }
  return result;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}