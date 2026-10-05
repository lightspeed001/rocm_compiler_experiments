// GEMM with LDS (AMD's shared memory)
__global__ void gemm_lds_opt(
    const float* A,
    const float* B,
    float* C,
    int M, int K, int N
) {
    __shared__ float A_lds[16][16];  // LDS for A
    __shared__ float B_lds[16][16];  // LDS for B

    int row = blockIdx.x * 16 + threadIdx.x;
    int col = blockIdx.y * 16 + threadIdx.y;

    float sum = 0.0f;
    for (int t = 0; t < (K + 15) / 16; ++t) {
        // Load A into LDS (bank-aware)
        if (row < M && (t * 16 + threadIdx.y) < K) {
            A_lds[threadIdx.y][threadIdx.x] = A[row * K + t * 16 + threadIdx.y];
        }
        // Load B into LDS (transposed for coalescing)
        if (col < N && (t * 16 + threadIdx.x) < K) {
            B_lds[threadIdx.x][threadIdx.y] = B[(t * 16 + threadIdx.x) * N + col];
        }
        __syncthreads();

        // Compute partial sum
        for (int i = 0; i < 16; ++i) {
            sum += A_lds[i][threadIdx.x] * B_lds[threadIdx.y][i];
        }
        __syncthreads();
    }

    if (row < M && col < N) {
        C[row * N + col] = sum;
    }
}
