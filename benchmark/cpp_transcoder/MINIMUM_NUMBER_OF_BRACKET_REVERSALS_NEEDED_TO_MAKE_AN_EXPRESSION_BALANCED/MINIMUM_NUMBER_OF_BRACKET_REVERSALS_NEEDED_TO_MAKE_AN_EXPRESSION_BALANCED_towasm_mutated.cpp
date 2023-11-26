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
int f_gold ( string expr ) {
  int len = expr . length ( );
  if ( len % 2 ) return - 1;
  stack < char > s;
  for ( int i = 0;
  i < len;
  i ++ ) {
    if ( expr [ i ] == '}' && ! s . empty ( ) ) {
      if ( s . top ( ) == '{' ) s . pop ( );
      else s . push ( expr [ i ] );
    }
    else s . push ( expr [ i ] );
  }
  int red_len = s . size ( );
  int n = 0;
  while ( ! s . empty ( ) && s . top ( ) == '{' ) {
    s . pop ( );
    n ++;
  }
  return ( red_len / 2 + n % 2 );
}


int main(void) {
		f_gold("cd");
}