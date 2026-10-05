// MFMA for FP16 GEMM (MI200/MI300)
__global__ void gemm_mfma_fp16(
    const half* A,  // FP16 input
    const half* B,
    float* C,       // FP32 output (accumulation)
    int M, int K, int N
) {
    // Each workgroup computes a 64x64 output tile
    int row = blockIdx.x * 64 + (threadIdx.x / 4) * 16;
    int col = blockIdx.y * 64 + (threadIdx.x % 4) * 16;

    // MFMA requires aligned pointers
    const half* A_aligned = __builtin_assume_aligned(A, 256);
    const half* B_aligned = __builtin_assume_aligned(B, 256);

    // MFMA intrinsic: 16x16x16 FP16 → FP32
    for (int t = 0; t < K; t += 16) {
        float4 a_vec[4], b_vec[4];
        // Load A and B (simplified; actual loading requires care)
        for (int i = 0; i < 4; ++i) {
            a_vec[i] = __builtin_amdgcn_lds_f32x4(...);
            b_vec[i] = __builtin_amdgcn_lds_f32x4(...);
        }

        // MFMA: C += A * B (16x16x16)
        __builtin_amdgcn_mfma_f32_16x16x16_f16(
            &C[row * N + col],
            &A_aligned[row * K + t],
            &B_aligned[t * N + col],
            &C[row * N + col]
        );
    }
}
