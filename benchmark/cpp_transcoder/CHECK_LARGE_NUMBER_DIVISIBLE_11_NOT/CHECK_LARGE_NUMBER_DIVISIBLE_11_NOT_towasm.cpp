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
  int n = str . length ( );
  int oddDigSum = 0, evenDigSum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( i % 2 == 0 ) oddDigSum += ( str [ i ] - '0' );
    else evenDigSum += ( str [ i ] - '0' );
  }
  return ( ( oddDigSum - evenDigSum ) % 11 == 0 );
}


int main(void) {
		f_gold("ab");
}