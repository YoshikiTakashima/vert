
#include <iostream>
#include <cstdlib>
#include <string>
#include <vector>
#include <fstream>
#include <iomanip>

using namespace std;
unsigned int f_gold ( unsigned int x ) {
  unsigned int even_bits = x & 0xAAAAAAAA;
  unsigned int odd_bits = x & 0x55555555;
  even_bits >>= 1;
  odd_bits <<= 1;
  return ( even_bits | odd_bits );
}


