#include "format.hpp"
#include <cstdio>
#include <errno.h>
#include <cstdlib>
#include <unistd.h>

#include <capabilities.hpp>

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
