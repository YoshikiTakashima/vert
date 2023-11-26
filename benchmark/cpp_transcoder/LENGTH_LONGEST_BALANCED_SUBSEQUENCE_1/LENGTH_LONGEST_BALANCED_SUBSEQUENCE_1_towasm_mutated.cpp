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
int f_gold ( char s [ ], int n ) {
  int invalidOpenBraces = 0;
  int invalidCloseBraces = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( s [ i ] == '(' ) {
      invalidOpenBraces ++;
    }
    else {
      if ( invalidOpenBraces == 0 ) {
        invalidCloseBraces ++;
      }
      else {
        invalidOpenBraces --;
      }
    }
  }
  return ( n - ( invalidOpenBraces + invalidCloseBraces ) );
}


int main(void) {
	char xv[] = {'c','d'};
	f_gold(xv,29);
}