extern crate llvm;

use llvm::{Builder, Context, Module, PassManager};

fn main() {
    let ctx = Context::new();
    let module = Module::new("main", &ctx);
    let builder = Builder::new(&ctx);

    let mut fpm = PassManager::new_func_pass(&module);

    fpm.add_instruction_combining()
        .add_reassociate()
        .add_gvn()
        .add_cfg();

    module
        .write_bitcode("out.bc")
        .expect("Couldn't write to file");

    module.verify().unwrap();

    println!("{:?}", module);
}
