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
int f_gold ( int n, int k ) {
  int eggFloor [ n + 1 ] [ k + 1 ];
  int res;
  int i, j, x;
  for ( i = 1;
  i <= n;
  i ++ ) {
    eggFloor [ i ] [ 1 ] = 1;
    eggFloor [ i ] [ 0 ] = 0;
  }
  for ( j = 1;
  j <= k;
  j ++ ) eggFloor [ 1 ] [ j ] = j;
  for ( i = 2;
  i <= n;
  i ++ ) {
    for ( j = 2;
    j <= k;
    j ++ ) {
      eggFloor [ i ] [ j ] = INT_MAX;
      for ( x = 1;
      x <= j;
      x ++ ) {
        res = 1 + max ( eggFloor [ i - 1 ] [ x - 1 ], eggFloor [ i ] [ j - x ] );
        if ( res < eggFloor [ i ] [ j ] ) eggFloor [ i ] [ j ] = res;
      }
    }
  }
  return eggFloor [ n ] [ k ];
}


int main(void) {
		f_gold(1,2);
}