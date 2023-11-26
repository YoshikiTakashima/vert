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
void f_gold ( char s1 [ ], char s2 [ ], int index = 0 ) {
  s2 [ index ] = s1 [ index ];
  if ( s1 [ index ] == '\0' ) return;
  f_gold ( s1, s2, index + 1 );
}


int main(void) {
	char xv[] = {'a','b'};
char yq[] = {'a','b'};
	f_gold(xv,yq,3);;
}