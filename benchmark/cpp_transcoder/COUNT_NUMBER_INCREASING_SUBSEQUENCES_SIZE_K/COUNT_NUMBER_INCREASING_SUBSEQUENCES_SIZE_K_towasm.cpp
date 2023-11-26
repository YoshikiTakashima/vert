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
int f_gold ( int arr [ ], int n, int k ) {
  int dp [ k ] [ n ], sum = 0;
  memset ( dp, 0, sizeof ( dp ) );
  for ( int i = 0;
  i < n;
  i ++ ) dp [ 0 ] [ i ] = 1;
  for ( int l = 1;
  l < k;
  l ++ ) {
    for ( int i = l;
    i < n;
    i ++ ) {
      dp [ l ] [ i ] = 0;
      for ( int j = l - 1;
      j < i;
      j ++ ) {
        if ( arr [ j ] < arr [ i ] ) dp [ l ] [ i ] += dp [ l - 1 ] [ j ];
      }
    }
  }
  for ( int i = k - 1;
  i < n;
  i ++ ) sum += dp [ k - 1 ] [ i ];
  return sum;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}