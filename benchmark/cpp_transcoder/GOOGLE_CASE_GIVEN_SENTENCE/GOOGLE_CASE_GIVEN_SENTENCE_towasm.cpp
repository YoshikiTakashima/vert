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
string f_gold ( string s ) {
  int n = s . length ( );
  s [ 0 ] = tolower ( s [ 0 ] );
  for ( int i = 1;
  i < n;
  i ++ ) {
    if ( s [ i ] == ' ' && i < n ) {
      s [ i + 1 ] = tolower ( s [ i + 1 ] );
      i ++;
    }
    else s [ i ] = toupper ( s [ i ] );
  }
  return s;
}


int main(void) {
		f_gold("ab");
}