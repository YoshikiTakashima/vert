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
int f_gold ( string num ) {
  int series [ ] = {
    1, 3, 2, - 1, - 3, - 2 };
    int series_index = 0;
    int result = 0;
    for ( int i = num . size ( ) - 1;
    i >= 0;
    i -- ) {
      int digit = num [ i ] - '0';
      result += digit * series [ series_index ];
      series_index = ( series_index + 1 ) % 6;
      result %= 7;
    }
    if ( result < 0 ) result = ( result + 7 ) % 7;
    return result;
  }
  

int main(void) {
		f_gold("ab");
}