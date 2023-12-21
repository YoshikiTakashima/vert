typedef signed long int __int64_t;


typedef __int64_t int64_t;
typedef struct _IO_FILE FILE;
#define NULL ((void *)0)
#define SEEK_END	2

static int64_t FileSize(const char* path) {
  FILE* f = fopen(path, "rb");
  int64_t retval;
  if (f == NULL) {
    return -1;
  }
  if (fseek(f, 0L, SEEK_END) != 0) {
    fclose(f);
    return -1;
  }
  retval = ftell(f);
  if (fclose(f) != 0) {
    return -1;
  }
  return retval;
}

int main(){
  
}