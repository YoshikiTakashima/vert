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

int f_gold ( int n, int k ) {
  while ( n % 2 == 0 ) {
    k --;
    n = n / 2;
    if ( k == 0 ) return 2;
  }
  for ( int i = 3;
  i <= sqrt ( n );
  i = i + 2 ) {
    while ( n % i == 0 ) {
      if ( k == 1 ) return i;
      k --;
      n = n / i;
    }
  }
  if ( n > 2 && k == 1 ) return n;
  return - 1;
}


int f_filled ( int n, int k ) {}

int main(void) {
    int n_success = 0;
    int param0[] = {94,99,64,27,24,84,69,69,22,39};
    int param1[] = {0,1,3,3,4,6,98,39,60,57};
    for(int i = 0; i < len(param0); ++i)
    {
        if(f_filled(param0[i],param1[i]) == f_gold(param0[i],param1[i]))
        {
            n_success+=1;
        }
	break;
	}
    printf("#Results:", " ", n_success, ", ", len(param0));
    return 0;
}