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
int f_gold ( int input, int unlock_code ) {
  int rotation = 0;
  int input_digit, code_digit;
  while ( input || unlock_code ) {
    input_digit = input % 10;
    code_digit = unlock_code % 10;
    rotation += min ( abs ( input_digit - code_digit ), 10 - abs ( input_digit - code_digit ) );
    input /= 10;
    unlock_code /= 10;
  }
  return rotation;
}


int main(void) {
		f_gold(1,29);
}