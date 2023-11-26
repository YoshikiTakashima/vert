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
int f_gold ( string str ) {
  int res = str [ 0 ] - '0';
  for ( int i = 1;
  i < str . length ( );
  i ++ ) {
    if ( str [ i ] == '0' || str [ i ] == '1' || res < 2 ) res += ( str [ i ] - '0' );
    else res *= ( str [ i ] - '0' );
  }
  return res;
}


int main(void) {
		f_gold("ab");
}