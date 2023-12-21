#define BROTLI_TRUE 1
#define BROTLI_FALSE 0


static int ParseInt(const char* s, int low, int high, int* result) {
  int value = 0;
  int i;
  for (i = 0; i < 5; ++i) {
    char c = s[i];
    if (c == 0) break;
    if (s[i] < '0' || s[i] > '9') return BROTLI_FALSE;
    value = (10 * value) + (c - '0');
  }
  if (i == 0) return BROTLI_FALSE;
  if (i > 1 && s[0] == '0') return BROTLI_FALSE;
  if (s[i] != 0) return BROTLI_FALSE;
  if (value < low || value > high) return BROTLI_FALSE;
  *result = value;
  return BROTLI_TRUE;
}

int main(){
  
}