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
int f_gold ( char s [ ] ) {
  int n = strlen ( s );
  int count = 0;
  for ( int i = 0;
  i < n;
  ++ i ) if ( s [ i ] == '4' || s [ i ] == '8' || s [ i ] == '0' ) count ++;
  for ( int i = 0;
  i < n - 1;
  ++ i ) {
    int h = ( s [ i ] - '0' ) * 10 + ( s [ i + 1 ] - '0' );
    if ( h % 4 == 0 ) count = count + i + 1;
  }
  return count;
}


int main(void) {
	char xv[] = {'c','d'};
	f_gold(xv);
}