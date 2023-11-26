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
  int i = str . length ( ) - 1;
  int start, end = i + 1;
  string result = "";
  while ( i >= 0 ) {
    if ( str [ i ] == ' ' ) {
      start = i + 1;
      while ( start != end ) result += str [ start ++ ];
      result += ' ';
      end = i;
    }
    i --;
  }
  start = 0;
  while ( start != end ) result += str [ start ++ ];
  return result;
}


int main(void) {
		f_gold("ab");
}