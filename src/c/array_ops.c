#include "array_ops.h"

// alignment of src, dst = 32
void array_copy(uint8_t *dst, const uint8_t *src, uint32_t len) {
  asm volatile
    (
     "cld;\n\t"
     "rep movsb;\n\t"
     : "+S"(src), "+D"(dst), "+c"(len)
     : 
     : "memory"
     );
}

// len % 64 == 0
void array_xor(uint8_t *dst, const uint8_t *src, uint32_t len) {
  asm volatile
    (
     "LOOP%=:\n\t"
     "vmovdqa (%1), %%ymm0;\n\t"
     "vmovdqa 32(%1), %%ymm1;\n\t"
     "vpxor (%0), %%ymm0, %%ymm0;\n\t"
     "vpxor 32(%0), %%ymm1, %%ymm1;\n\t"
     "vmovdqa %%ymm0, (%0);\n\t"
     "vmovdqa %%ymm1, 32(%0);\n\t"
     "add $64, %0;\n\t"
     "add $64, %1;\n\t"
     "sub $64, %2;\n\t"
     "jnz LOOP%=;"
     "sfence;"
     : "+r"(dst), "+r"(src), "+r"(len)
     : 
     : "ymm0", "ymm1", "memory"
     );
}
