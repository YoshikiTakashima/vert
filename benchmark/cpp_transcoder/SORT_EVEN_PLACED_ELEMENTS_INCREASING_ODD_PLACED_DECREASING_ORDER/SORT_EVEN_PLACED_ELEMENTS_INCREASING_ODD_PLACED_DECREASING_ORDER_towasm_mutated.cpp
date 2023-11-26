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
void f_gold ( int arr [ ], int n ) {
  vector < int > evenArr;
  vector < int > oddArr;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( ! ( i % 2 ) ) evenArr . push_back ( arr [ i ] );
    else oddArr . push_back ( arr [ i ] );
  }
  sort ( evenArr . begin ( ), evenArr . end ( ) );
  sort ( oddArr . begin ( ), oddArr . end ( ), greater < int > ( ) );
  int i = 0;
  for ( int j = 0;
  j < evenArr . size ( );
  j ++ ) arr [ i ++ ] = evenArr [ j ];
  for ( int j = 0;
  j < oddArr . size ( );
  j ++ ) arr [ i ++ ] = oddArr [ j ];
}



int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);;
}