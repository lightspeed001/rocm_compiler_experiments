clang -S -emit-llvm kernel.c -o kernel.ll

llc -mtriple=amdgcn-amd-amdhsa -mcpu=gfx906 kernel.ll -o kernel.s

