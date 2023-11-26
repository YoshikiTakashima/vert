
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
int f_gold ( int array [ ], int start, int end ) {
  if ( start > end ) return end + 1;
  if ( start != array [ start ] ) return start;
  int mid = ( start + end ) / 2;
  if ( array [ mid ] == mid ) return f_gold ( array, mid + 1, end );
  return f_gold ( array, start, mid );
}


