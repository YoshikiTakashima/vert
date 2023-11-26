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
  int n = s . length ( );
  int lps [ n ];
  lps [ 0 ] = 0;
  int len = 0;
  int i = 1;
  while ( i < n ) {
    if ( s [ i ] == s [ len ] ) {
      len ++;
      lps [ i ] = len;
      i ++;
    }
    else {
      if ( len != 0 ) {
        len = lps [ len - 1 ];
      }
      else {
        lps [ i ] = 0;
        i ++;
      }
    }
  }
  int res = lps [ n - 1 ];
  return ( res > n / 2 ) ? n / 2 : res;
}


int main(void) {
		f_gold("cd");
}