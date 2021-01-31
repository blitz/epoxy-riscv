#include "plic.hpp"
#include "resources.hpp"

plic const global_plic { plic_reg, plic_ndev };

plic const& plic::global()
{
  return global_plic;
}
