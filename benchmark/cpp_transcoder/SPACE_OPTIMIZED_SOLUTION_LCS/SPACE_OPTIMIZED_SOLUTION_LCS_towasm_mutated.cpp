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
  int m = X . length ( ), n = Y . length ( );
  int L [ 2 ] [ n + 1 ];
  bool bi;
  for ( int i = 0;
  i <= m;
  i ++ ) {
    bi = i & 1;
    for ( int j = 0;
    j <= n;
    j ++ ) {
      if ( i == 0 || j == 0 ) L [ bi ] [ j ] = 0;
      else if ( X [ i - 1 ] == Y [ j - 1 ] ) L [ bi ] [ j ] = L [ 1 - bi ] [ j - 1 ] + 1;
      else L [ bi ] [ j ] = max ( L [ 1 - bi ] [ j ], L [ bi ] [ j - 1 ] );
    }
  }
  return L [ bi ] [ n ];
}


int main(void) {
		f_gold("cd","cb");
}