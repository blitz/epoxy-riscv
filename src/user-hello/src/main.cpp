#include <cstdio>

struct hello_world_exception {
};

int test()
{
  throw hello_world_exception {};
}

int main()
{
  printf("Hello World!\n");

  test();

  return 0;
}
