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
  int l = s . length ( );
  string s1 = "";
  bool isEven = ( l % 2 == 0 ) ? 1 : 0;
  for ( int i = 0;
  i < l;
  i += 2 ) {
    if ( isEven ) {
      s1 = s [ i ] + s1;
      s1 += s [ i + 1 ];
    }
    else {
      if ( l - i > 1 ) {
        s1 += s [ i ];
        s1 = s [ i + 1 ] + s1;
      }
      else {
        s1 += s [ i ];
      }
    }
  }
  return s1;
}


int main(void) {
		f_gold("cd");
}