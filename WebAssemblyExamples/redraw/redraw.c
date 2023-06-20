// clang --target=wasm32 -O3 -flto --no-standard-libraries -Wl,--export-all -Wl,--no-entry -Wl,--lto-O3 -o redraw.wasm redraw.c

#define WASM_EXPORT __attribute__((visibility("default")))

// Memory management
extern unsigned char __heap_base;
unsigned int bump_pointer = (int) &__heap_base;

WASM_EXPORT
void *pmalloc(int n) {
  unsigned int r = bump_pointer;
  bump_pointer += n;
  return (void *)r;
}

WASM_EXPORT
void pfree(int n) {
  bump_pointer -= n;
}

double fn1(double x) {
    return x*x;
}

double fn2(double x) {
    return x+x;
}

void redraw(double width, double height) {
    int n = 100;
    double x0 = 3.0;
    double step = 0.01;
    for(int j = 0; j < n; j++) {
        double x = x0 + step*((double) j);
        double y1 = fn1(x);
        double *p = pmalloc(8);
        *p = y1;
    }
}