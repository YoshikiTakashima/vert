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
int f_gold ( int n ) {
  int result = 0;
  for ( int i = 1;
  i <= 9;
  i ++ ) {
    stack < int > s;
    if ( i <= n ) {
      s . push ( i );
      result ++;
    }
    while ( ! s . empty ( ) ) {
      int tp = s . top ( );
      s . pop ( );
      for ( int j = tp % 10;
      j <= 9;
      j ++ ) {
        int x = tp * 10 + j;
        if ( x <= n ) {
          s . push ( x );
          result ++;
        }
      }
    }
  }
  return result;
}


int main(void) {
		f_gold(1);
}