// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//


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

int f_gold ( int a, int b, int d ) {
  int temp = a;
  a = min ( a, b );
  b = max ( temp, b );
  if ( d >= b ) return ( d + b - 1 ) / b;
  if ( d == 0 ) return 0;
  if ( d == a ) return 1;
  return 2;
}


int f_filled ( int a, int b, int d ) {}

int main(void) {
    int n_success = 0;
    int param0[] = {35,85,22,8,12,58,65,10,23,5};
    int param1[] = {8,55,23,43,64,25,4,95,13,50};
    int param2[] = {77,33,64,29,11,26,28,55,54,71};
    for(int i = 0; i < len(param0); ++i)
    {
        if(f_filled(param0[i],param1[i],param2[i]) == f_gold(param0[i],param1[i],param2[i]))
        {
            n_success+=1;
        }
	break;
	}
    printf("#Results:", " ", n_success, ", ", len(param0));
    return 0;
}