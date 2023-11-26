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
  int aCount = 0;
  int bCount = 0;
  int cCount = 0;
  for ( unsigned int i = 0;
  i < s . size ( );
  i ++ ) {
    if ( s [ i ] == 'a' ) aCount = ( 1 + 2 * aCount );
    else if ( s [ i ] == 'b' ) bCount = ( aCount + 2 * bCount );
    else if ( s [ i ] == 'c' ) cCount = ( bCount + 2 * cCount );
  }
  return cCount;
}


int main(void) {
		f_gold("ab");
}