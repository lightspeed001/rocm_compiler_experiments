use llvm_sys::*;
use std::ffi::CString;

fn main() {
    unsafe {
        // Initialize LLVM
        llvm_sys::core::LLVMLinkInMCJIT();
        llvm_sys::target::LLVMInitializeAMDGPUTarget();
        llvm_sys::target::LLVMInitializeAMDGPUTargetInfo();
        llvm_sys::target::LLVMInitializeAMDGPUTargetMC();
        llvm_sys::target::LLVMInitializeAMDGPUAsmPrinter();

        // Create context and module
        let context = llvm_sys::core::LLVMContextCreate();
        let module = llvm_sys::core::LLVMModuleCreateWithNameInContext(
            CString::new("kernel").unwrap().as_ptr(),
            context,
        );

        // Set target triple and data layout
        let triple = CString::new("amdgcn-amd-amdhsa").unwrap();
        llvm_sys::core::LLVMModuleSetTargetTriple(module, triple.as_ptr());
        llvm_sys::core::LLVMModuleSetDataLayout(
            module,
            CString::new("").unwrap().as_ptr(),
        );

        // Parse LLVM IR (or build it programmatically)
        let llvm_ir = CString::new(
            "target datalayout = \"e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-v16:16:16-v24:32:32-v32:32:32-v48:64:64-v96:128:128-v192:256:256-v256:256:256-v512:512:512-v1024:1024:1024\"\n"
            .to_owned() +
            "target triple = \"amdgcn-amd-amdhsa\"\n" +
            "define void @add(float* %a, float* %b, float* %c, i32 %n) {\n" +
            "  ret void\n" +
            "}\n"
        ).unwrap();

        let mut error = std::ptr::null_mut();
        llvm_sys::ir_reader::LLVMParseIRInContext(
            context,
            llvm_ir.as_ptr(),
            llvm_ir.as_bytes().len(),
            &mut module,
            &mut error,
        );
        if !error.is_null() {
            eprintln!("LLVM IR parse error: {:?}", CString::from_raw(error));
            return;
        }

        // Create target machine for AMD GCN
        let mut target = std::ptr::null_mut();
        let mut target_error = std::ptr::null_mut();
        if llvm_sys::target::LLVMGetTargetFromTriple(
            triple.as_ptr(),
            &mut target,
            &mut target_error,
        ) != 0
        {
            eprintln!("Target error: {:?}", CString::from_raw(target_error));
            return;
        }

        let tm = llvm_sys::target_machine::LLVMCreateTargetMachine(
            target,
            CString::new("gfx906").unwrap().as_ptr(), // GPU architecture
            CString::new("").unwrap().as_ptr(),         // CPU features
            llvm_sys::target_machine::LLVMCodeGenLevel::LLVMCodeGenLevelAggressive,
            llvm_sys::target_machine::LLVMRelocMode::LLVMRelocStatic,
            llvm_sys::target_machine::LLVMCodeModel::LLVMCodeModelDefault,
        );

        // Compile to AMD GCN ISA
        let mut isa = std::ptr::null_mut();
        let mut isa_size = 0;
        llvm_sys::target_machine::LLVMTargetMachineEmitToString(
            tm,
            module,
            llvm_sys::target_machine::LLVMObjectFile,
            &mut isa,
            &mut isa_size,
        );

        println!(
            "Generated AMD GCN ISA:\n{}",
            CString::from_raw(isa).to_string_lossy()
        );

        // Cleanup
        llvm_sys::target_machine::LLVMDisposeTargetMachine(tm);
        llvm_sys::core::LLVMDisposeModule(module);
        llvm_sys::core::LLVMContextDispose(context);
    }
}
