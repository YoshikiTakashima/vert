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
char f_gold ( string strA, string strB ) {
  int res = 0, i;
  for ( i = 0;
  i < strA . length ( );
  i ++ ) {
    res ^= strA [ i ];
  }
  for ( i = 0;
  i < strB . length ( );
  i ++ ) {
    res ^= strB [ i ];
  }
  return ( ( char ) ( res ) );
}


int main(void) {
		f_gold("cd","cb");
}