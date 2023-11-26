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
int f_gold ( string s1, string s2 ) {
  int n = s1 . length ( );
  int m = s2 . length ( );
  bool dp [ n + 1 ] [ m + 1 ];
  for ( int i = 0;
  i <= n;
  i ++ ) {
    for ( int j = 0;
    j <= m;
    j ++ ) {
      dp [ i ] [ j ] = 0;
    }
  }
  dp [ 0 ] [ 0 ] = 1;
  for ( int i = 0;
  i < s1 . length ( );
  i ++ ) {
    for ( int j = 0;
    j <= s2 . length ( );
    j ++ ) {
      if ( dp [ i ] [ j ] ) {
        if ( j < s2 . length ( ) && ( toupper ( s1 [ i ] ) == s2 [ j ] ) ) dp [ i + 1 ] [ j + 1 ] = 1;
        if ( ! isupper ( s1 [ i ] ) ) dp [ i + 1 ] [ j ] = 1;
      }
    }
  }
  return ( dp [ n ] [ m ] );
}


int main(void) {
		f_gold("cd","cb");
}