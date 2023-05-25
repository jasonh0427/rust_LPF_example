#include "LowPassFilter.hpp"

int main() {
    auto pLowPassFilter = lpf::create(48000, 2000);
    lpf::set_f0(pLowPassFilter, 1500);
    lpf::destroy(pLowPassFilter);
}