
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int W, int wt [ ], int val [ ], int n ) {
  if ( n == 0 || W == 0 ) return 0;
  if ( wt [ n - 1 ] > W ) return f_gold ( W, wt, val, n - 1 );
  else return max ( val [ n - 1 ] + f_gold ( W - wt [ n - 1 ], wt, val, n - 1 ), f_gold ( W, wt, val, n - 1 ) );
}


