__attribute__((kernel))
void add(__global float *a, __global float *b, __global float *c, int n) {
    int idx = get_global_id(0);
    if (idx < n) c[idx] = a[idx] + b[idx];
}
