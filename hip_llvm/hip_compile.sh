hipcc --amdgcn-target=gfx906 -S kernel.hip -o kernel.s

rocminfo | grep gfx
