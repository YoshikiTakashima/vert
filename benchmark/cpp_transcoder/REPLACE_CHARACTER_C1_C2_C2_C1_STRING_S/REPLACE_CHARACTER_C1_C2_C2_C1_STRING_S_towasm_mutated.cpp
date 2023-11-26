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
string f_gold ( string s, char c1, char c2 ) {
  int l = s . length ( );
  for ( int i = 0;
  i < l;
  i ++ ) {
    if ( s [ i ] == c1 ) s [ i ] = c2;
    else if ( s [ i ] == c2 ) s [ i ] = c1;
  }
  return s;
}


int main(void) {
		f_gold("cd",'d','c');
}