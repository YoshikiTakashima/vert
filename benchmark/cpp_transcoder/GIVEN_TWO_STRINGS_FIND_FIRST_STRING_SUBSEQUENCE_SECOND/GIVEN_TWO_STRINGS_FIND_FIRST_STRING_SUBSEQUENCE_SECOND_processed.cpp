
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( char str1 [ ], char str2 [ ], int m, int n ) {
  if ( m == 0 ) return 1;
  if ( n == 0 ) return 0;
  if ( str1 [ m - 1 ] == str2 [ n - 1 ] ) return f_gold ( str1, str2, m - 1, n - 1 );
  return f_gold ( str1, str2, m, n - 1 );
}


