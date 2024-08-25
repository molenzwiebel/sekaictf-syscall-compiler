use std::sync::Mutex;

use assembler::AssemblerContext;
use clap::Parser;
use consts::preg_to_name;
use insn::ProgramBuilder;
use once_cell::sync::OnceCell;
use rand::{rngs::StdRng, SeedableRng};
use regalloc2::{Block, Function};
use syscalls::SyscallIndices;

mod assembler;
mod chall;
mod complex;
mod consts;
mod insn;
mod regalloc;
mod simple;
mod syscalls;
mod util;
mod zwimpl;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub static RAND: OnceCell<Mutex<StdRng>> = OnceCell::new();

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional output path.
    #[arg(short, long, default_value = "out.exe")]
    output: String,

    /// Optional seed to use for random number generation.
    #[arg(short, long)]
    seed: Option<u64>,

    /// Path to syscall numbers to use.
    #[arg(short, long)]
    syscall_numbers: Option<String>,

    /// Version required for this to run.
    #[arg(short, long)]
    version: String,

    /// Dump out the generated program before assembling.
    #[arg(long)]
    dump_program: bool,

    /// Dump out the generated program after regalloc.
    #[arg(long)]
    dump_regalloc: bool,
}

fn main() {
    let args = Cli::parse();
    match args.seed {
        Some(seed) => {
            RAND.set(Mutex::new(StdRng::seed_from_u64(seed))).unwrap();
        }
        None => {
            RAND.set(Mutex::new(StdRng::from_entropy())).unwrap();
        }
    }

    let mut builder = ProgramBuilder::new();
    builder.challenge("sy5c4ll_m3_m4yb3_2c94234", &args.version);

    let program = builder.build();

    if args.dump_program {
        for i in 0..program.num_blocks() {
            let block = Block(i as u32);

            let preds = program.block_preds(block);
            let succs = program.block_succs(block);
            println!(
                "== block {} | pred = {:?} | succ = {:?} ==",
                i, preds, succs
            );

            let range = program.block_insns(block);
            for inst in range.iter() {
                let insn = program.insn_from_inst(inst);
                let clobbers = program
                    .inst_clobbers(inst)
                    .into_iter()
                    .map(|x| preg_to_name(x))
                    .collect::<Vec<_>>();
                let operands = program.inst_operands(inst);

                println!(
                    "insn {} | {:?} | operands={:?} | clobbers={:?}",
                    inst.0, insn.0, operands, clobbers
                );
            }

            println!();
        }
    }

    let result = program.run();

    if args.dump_regalloc {
        for i in 0..program.num_blocks() {
            let block = Block(i as u32);

            let preds = program.block_preds(block);
            let succs = program.block_succs(block);
            println!(
                "== block {} | pred = {:?} | succ = {:?} ==",
                i, preds, succs
            );

            for inst_or_edit in result.block_insts_and_edits(&program, block) {
                match inst_or_edit {
                    regalloc2::InstOrEdit::Inst(inst) => {
                        let insn = program.insn_from_inst(inst);
                        let clobbers = program
                            .inst_clobbers(inst)
                            .into_iter()
                            .map(|x| preg_to_name(x))
                            .collect::<Vec<_>>();
                        let operands = program.inst_operands(inst);
                        let allocations = result.inst_allocs(inst);

                        println!(
                            "insn {} | {:?} | operands={:?} | clobbers={:?} | allocs={:?}",
                            inst.0, insn.0, operands, clobbers, allocations
                        );
                    }
                    regalloc2::InstOrEdit::Edit(e) => println!("{:?}", e),
                }
            }

            println!();
        }
    }

    let mut assembler = match args.syscall_numbers {
        Some(path) => AssemblerContext::new(SyscallIndices::from_table(&path)),
        None => AssemblerContext::new(SyscallIndices::from_ntdll("C:/Windows/System32/ntdll.dll")),
    };
    assembler.assemble_insns(&program, &result);

    let pe_out = assembler.finalize_and_build_pe();
    std::fs::write(&args.output, pe_out).unwrap();
}
