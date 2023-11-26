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
int f_gold ( string str ) {
  int n = str . length ( );
  int dp [ n + 1 ] [ n + 1 ];
  for ( int i = 0;
  i <= n;
  i ++ ) for ( int j = 0;
  j <= n;
  j ++ ) dp [ i ] [ j ] = 0;
  for ( int i = 1;
  i <= n;
  i ++ ) {
    for ( int j = 1;
    j <= n;
    j ++ ) {
      if ( str [ i - 1 ] == str [ j - 1 ] && i != j ) dp [ i ] [ j ] = 1 + dp [ i - 1 ] [ j - 1 ];
      else dp [ i ] [ j ] = max ( dp [ i ] [ j - 1 ], dp [ i - 1 ] [ j ] );
    }
  }
  return dp [ n ] [ n ];
}


int main(void) {
		f_gold("cd");
}