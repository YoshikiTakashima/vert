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
int f_gold ( string X, string Y ) {
  int m = X . length ( );
  int n = Y . length ( );
  int result = 0;
  int len [ 2 ] [ n ];
  int currRow = 0;
  for ( int i = 0;
  i <= m;
  i ++ ) {
    for ( int j = 0;
    j <= n;
    j ++ ) {
      if ( i == 0 || j == 0 ) {
        len [ currRow ] [ j ] = 0;
      }
      else if ( X [ i - 1 ] == Y [ j - 1 ] ) {
        len [ currRow ] [ j ] = len [ 1 - currRow ] [ j - 1 ] + 1;
        result = max ( result, len [ currRow ] [ j ] );
      }
      else {
        len [ currRow ] [ j ] = 0;
      }
    }
    currRow = 1 - currRow;
  }
  return result;
}


int main(void) {
		f_gold("cd","cb");
}