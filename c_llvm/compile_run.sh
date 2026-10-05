# Link against LLVM (adjust paths for your system)
gcc llvm_to_gcn.c -o llvm_to_gcn -lLLVM-16 -I/usr/lib/llvm-16/include -L/usr/lib/llvm-16/lib
./llvm_to_gcn
