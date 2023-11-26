
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
long long f_gold ( long long a, long long b, long long mod ) {
  long long res = 0;
  a %= mod;
  while ( b ) {
    if ( b & 1 ) res = ( res + a ) % mod;
    a = ( 2 * a ) % mod;
    b >>= 1;
  }
  return res;
}


