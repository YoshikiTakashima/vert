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
int f_gold ( const vector < vector < int > > & m ) {
  int rows = m . size ( );
  if ( rows == 0 ) return 0;
  int columns = m [ 0 ] . size ( );
  for ( int y1 = 0;
  y1 < rows;
  y1 ++ ) for ( int x1 = 0;
  x1 < columns;
  x1 ++ ) if ( m [ y1 ] [ x1 ] == 1 ) for ( int y2 = y1 + 1;
  y2 < rows;
  y2 ++ ) for ( int x2 = x1 + 1;
  x2 < columns;
  x2 ++ ) if ( m [ y1 ] [ x2 ] == 1 && m [ y2 ] [ x1 ] == 1 && m [ y2 ] [ x2 ] == 1 ) return 1;
  return 0;
}


int main(void) {
		f_gold(param0[i]);
}