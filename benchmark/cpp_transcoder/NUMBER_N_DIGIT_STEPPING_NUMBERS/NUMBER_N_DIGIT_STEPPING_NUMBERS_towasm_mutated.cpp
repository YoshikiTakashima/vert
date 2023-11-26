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
long long f_gold ( int n ) {
  int dp [ n + 1 ] [ 10 ];
  if ( n == 1 ) return 10;
  for ( int j = 0;
  j <= 9;
  j ++ ) dp [ 1 ] [ j ] = 1;
  for ( int i = 2;
  i <= n;
  i ++ ) {
    for ( int j = 0;
    j <= 9;
    j ++ ) {
      if ( j == 0 ) dp [ i ] [ j ] = dp [ i - 1 ] [ j + 1 ];
      else if ( j == 9 ) dp [ i ] [ j ] = dp [ i - 1 ] [ j - 1 ];
      else dp [ i ] [ j ] = dp [ i - 1 ] [ j - 1 ] + dp [ i - 1 ] [ j + 1 ];
    }
  }
  long long sum = 0;
  for ( int j = 1;
  j <= 9;
  j ++ ) sum += dp [ n ] [ j ];
  return sum;
}


int main(void) {
		f_gold(29);
}