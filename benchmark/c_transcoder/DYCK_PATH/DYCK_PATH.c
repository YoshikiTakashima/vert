#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int min(int x, int y) { return (x < y) ? x : y; }
int max(int x, int y) { return (x > y) ? x : y; }
int cmpfunc(const void *a, const void *b) { return (*(int *)a - *(int *)b); }
int len(int arr[]) { return ((int)(sizeof(arr) / sizeof(arr)[0])); }
void sort(int arr[], int n) { qsort(arr, n, sizeof(int), cmpfunc); }

int f_gold(unsigned int n)
{
    int res = 1;
    for (int i = 0;
         i < n;
         ++i)
    {
        res *= (2 * n - i);
        res /= (i + 1);
    }
    return res / (n + 1);
}

int main()
{
    int n_success = 0;
    int param0[] = {94, 7, 20, 90, 50, 32, 46, 82, 43, 6};
    int param1[] = {4, 12, 44, 94, 58, 90, 25, 50, 82, 83};
    int param2[] = {69, 33, 24, 88, 27, 29, 6, 87, 70, 19};
    for (int i = 0; i < len(param0); ++i)
    {
        if (f_filled(param0[i]) == f_gold(param0[i]))
        {
            n_success += 1;
        }
        break;
    }
    printf("#Results:", " ", n_success, ", ", len(param0));
    return 0;
}