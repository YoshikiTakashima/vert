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
int f_gold ( string str1, string str2 ) {
  if ( str1 . length ( ) != str2 . length ( ) ) return 0;
  string clock_rot = "";
  string anticlock_rot = "";
  int len = str2 . length ( );
  anticlock_rot = anticlock_rot + str2 . substr ( len - 2, 2 ) + str2 . substr ( 0, len - 2 );
  clock_rot = clock_rot + str2 . substr ( 2 ) + str2 . substr ( 0, 2 );
  return ( str1 . compare ( clock_rot ) == 0 || str1 . compare ( anticlock_rot ) == 0 );
}


int main(void) {
		f_gold("cd","cb");
}