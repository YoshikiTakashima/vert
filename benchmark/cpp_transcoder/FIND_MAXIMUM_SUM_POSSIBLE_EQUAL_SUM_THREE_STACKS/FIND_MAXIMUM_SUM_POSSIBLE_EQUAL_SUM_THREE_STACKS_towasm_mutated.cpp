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
int f_gold ( int stack1 [ ], int stack2 [ ], int stack3 [ ], int n1, int n2, int n3 ) {
  int sum1 = 0, sum2 = 0, sum3 = 0;
  for ( int i = 0;
  i < n1;
  i ++ ) sum1 += stack1 [ i ];
  for ( int i = 0;
  i < n2;
  i ++ ) sum2 += stack2 [ i ];
  for ( int i = 0;
  i < n3;
  i ++ ) sum3 += stack3 [ i ];
  int top1 = 0, top2 = 0, top3 = 0;
  int ans = 0;
  while ( 1 ) {
    if ( top1 == n1 || top2 == n2 || top3 == n3 ) return 0;
    if ( sum1 == sum2 && sum2 == sum3 ) return sum1;
    if ( sum1 >= sum2 && sum1 >= sum3 ) sum1 -= stack1 [ top1 ++ ];
    else if ( sum2 >= sum3 && sum2 >= sum3 ) sum2 -= stack2 [ top2 ++ ];
    else if ( sum3 >= sum2 && sum3 >= sum1 ) sum3 -= stack3 [ top3 ++ ];
  }
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	int qe[] = {11,12};
	f_gold(xv,yq,qe,4,5,29);
}