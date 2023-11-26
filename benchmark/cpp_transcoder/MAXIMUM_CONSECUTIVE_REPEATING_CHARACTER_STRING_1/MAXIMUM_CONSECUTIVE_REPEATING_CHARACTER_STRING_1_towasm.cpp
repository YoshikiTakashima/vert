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
char f_gold ( string str ) {
  int n = str . length ( );
  int count = 0;
  char res = str [ 0 ];
  int cur_count = 1;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( i < n - 1 && str [ i ] == str [ i + 1 ] ) cur_count ++;
    else {
      if ( cur_count > count ) {
        count = cur_count;
        res = str [ i ];
      }
      cur_count = 1;
    }
  }
  return res;
}


int main(void) {
		f_gold("ab");
}