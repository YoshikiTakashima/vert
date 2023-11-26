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
string f_gold ( string X, string Y ) {
  int m = X . length ( );
  int n = Y . length ( );
  int dp [ m + 1 ] [ n + 1 ];
  for ( int i = 0;
  i <= m;
  i ++ ) {
    for ( int j = 0;
    j <= n;
    j ++ ) {
      if ( i == 0 ) dp [ i ] [ j ] = j;
      else if ( j == 0 ) dp [ i ] [ j ] = i;
      else if ( X [ i - 1 ] == Y [ j - 1 ] ) dp [ i ] [ j ] = 1 + dp [ i - 1 ] [ j - 1 ];
      else dp [ i ] [ j ] = 1 + min ( dp [ i - 1 ] [ j ], dp [ i ] [ j - 1 ] );
    }
  }
  int index = dp [ m ] [ n ];
  string str;
  int i = m, j = n;
  while ( i > 0 && j > 0 ) {
    if ( X [ i - 1 ] == Y [ j - 1 ] ) {
      str . push_back ( X [ i - 1 ] );
      i --, j --, index --;
    }
    else if ( dp [ i - 1 ] [ j ] > dp [ i ] [ j - 1 ] ) {
      str . push_back ( Y [ j - 1 ] );
      j --, index --;
    }
    else {
      str . push_back ( X [ i - 1 ] );
      i --, index --;
    }
  }
  while ( i > 0 ) {
    str . push_back ( X [ i - 1 ] );
    i --, index --;
  }
  while ( j > 0 ) {
    str . push_back ( Y [ j - 1 ] );
    j --, index --;
  }
  reverse ( str . begin ( ), str . end ( ) );
  return str;
}


int main(void) {
		f_gold("cd","cb");
}