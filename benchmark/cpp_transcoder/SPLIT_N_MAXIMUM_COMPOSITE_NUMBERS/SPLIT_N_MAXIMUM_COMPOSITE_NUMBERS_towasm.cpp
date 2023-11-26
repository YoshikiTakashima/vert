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
  if ( n < 4 ) return - 1;
  int rem = n % 4;
  if ( rem == 0 ) return n / 4;
  if ( rem == 1 ) {
    if ( n < 9 ) return - 1;
    return ( n - 9 ) / 4 + 1;
  }
  if ( rem == 2 ) return ( n - 6 ) / 4 + 1;
  if ( rem == 3 ) {
    if ( n < 15 ) return - 1;
    return ( n - 15 ) / 4 + 2;
  }
}


int main(void) {
		f_gold(1);
}