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

long f_gold ( int n ) {
  long dp [ 2 ] [ n + 1 ];
  dp [ 0 ] [ 1 ] = 1;
  dp [ 1 ] [ 1 ] = 2;
  for ( int i = 2;
  i <= n;
  i ++ ) {
    dp [ 0 ] [ i ] = dp [ 0 ] [ i - 1 ] + dp [ 1 ] [ i - 1 ];
    dp [ 1 ] [ i ] = dp [ 0 ] [ i - 1 ] * 2 + dp [ 1 ] [ i - 1 ];
  }
  return dp [ 0 ] [ n ] + dp [ 1 ] [ n ];
}


long f_filled ( int n ) {}

int main(void) {
    int n_success = 0;
    int param0[] = {68,91,99,79,61,48,89,20,83,1};
    for(int i = 0; i < len(param0); ++i)
    {
        if(f_filled(param0[i]) == f_gold(param0[i]))
        {
            n_success+=1;
        }
	break;
	}
    printf("#Results:", " ", n_success, ", ", len(param0));
    return 0;
}