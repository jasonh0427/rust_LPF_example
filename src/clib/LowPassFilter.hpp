#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace lpf {

extern "C" {

/// create a lpf instance with a sample rate and cutoff frequency
/// The client is responsible for freeing the instance's memory when it's no longer required,
/// see `destroy()`.
///
struct LowPassFilter;

LowPassFilter *create(float fs, float f0);

/// Destroy a lpf instance
///
/// # Safety
///
/// The instance must have been previously created using `create()`.
void destroy(LowPassFilter *lpf);

/// filter the input with a given buffer
void process(LowPassFilter *lpf,
             const float *input_l,
             const float *input_r,
             float *output_l,
             float *output_r,
             uintptr_t sample_count);

void set_f0(LowPassFilter *lpf, float value);

void set_fs(LowPassFilter *lpf, float value);

} // extern "C"

} // namespace lpf
