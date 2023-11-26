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
int f_gold ( string s ) {
  if ( s . length ( ) == 0 ) cout << 0 << endl;
  int ans = 0;
  int o = 0, c = 0;
  for ( int i = 0;
  i < s . length ( );
  i ++ ) {
    if ( s [ i ] == '(' ) o ++;
    if ( s [ i ] == ')' ) c ++;
  }
  if ( o != c ) return - 1;
  int a [ s . size ( ) ];
  if ( s [ 0 ] == '(' ) a [ 0 ] = 1;
  else a [ 0 ] = - 1;
  if ( a [ 0 ] < 0 ) ans += abs ( a [ 0 ] );
  for ( int i = 1;
  i < s . length ( );
  i ++ ) {
    if ( s [ i ] == '(' ) a [ i ] = a [ i - 1 ] + 1;
    else a [ i ] = a [ i - 1 ] - 1;
    if ( a [ i ] < 0 ) ans += abs ( a [ i ] );
  }
  return ans;
}


int main(void) {
		f_gold("cd");
}