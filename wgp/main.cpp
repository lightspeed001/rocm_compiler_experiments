// Wavefront-level reduction for LayerNorm
__global__ void layer_norm_wave_opt(
    const float* input,
    const float* gamma,
    const float* beta,
    float* output,
    int N, int D,
    float eps
) {
    int row = blockIdx.x;
    if (row < N) {
        // Wavefront-level mean
        float sum = 0.0f;
        for (int col = threadIdx.x; col < D; col += 64) {
            sum += input[row * D + col];
        }
        // Wavefront shuffle (64 threads)
        for (int offset = 32; offset > 0; offset /= 2) {
            sum += __builtin_amdgcn_shfl_down(sum, offset);
        }
        float mean_val = sum / D;

        // Wavefront-level variance
        float sum_sq = 0.0f;
        for (int col = threadIdx.x; col < D; col += 64) {
            float val = input[row * D + col] - mean_val;
            sum_sq += val * val;
        }
        for (int offset = 32; offset > 0; offset /= 2) {
            sum_sq += __builtin_amdgcn_shfl_down(sum_sq, offset);
        }
        float var_val = rsqrtf(sum_sq / D + eps);

        // Normalize
        for (int col = threadIdx.x; col < D; col += 64) {
            float val = input[row * D + col];
            float norm_val = (val - mean_val) * var_val;
            output[row * D + col] = norm_val * gamma[col] + beta[col];
        }
    }
}
