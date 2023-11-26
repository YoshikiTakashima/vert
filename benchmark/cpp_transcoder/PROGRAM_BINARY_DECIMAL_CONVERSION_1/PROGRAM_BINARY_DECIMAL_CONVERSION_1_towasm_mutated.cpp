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
int f_gold ( string n ) {
  string num = n;
  int dec_value = 0;
  int base = 1;
  int len = num . length ( );
  for ( int i = len - 1;
  i >= 0;
  i -- ) {
    if ( num [ i ] == '1' ) dec_value += base;
    base = base * 2;
  }
  return dec_value;
}


int main(void) {
		f_gold("cd");
}