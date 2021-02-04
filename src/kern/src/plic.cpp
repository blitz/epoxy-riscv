#include "plic.hpp"

#include "resources.hpp"

plic const &plic::global()
{
  static plic const global_plic {plic_reg, plic_ndev};

  return global_plic;
}
