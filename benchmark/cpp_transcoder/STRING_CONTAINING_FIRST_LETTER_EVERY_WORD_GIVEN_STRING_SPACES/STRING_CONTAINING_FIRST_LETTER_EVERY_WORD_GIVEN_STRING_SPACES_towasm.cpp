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
string f_gold ( string str ) {
  string result = "";
  bool v = 1;
  for ( int i = 0;
  i < str . length ( );
  i ++ ) {
    if ( str [ i ] == ' ' ) v = 1;
    else if ( str [ i ] != ' ' && v == 1 ) {
      result . push_back ( str [ i ] );
      v = 0;
    }
  }
  return result;
}


int main(void) {
		f_gold("ab");
}