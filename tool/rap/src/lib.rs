#![feature(rustc_private)]
#![feature(control_flow_enum)]
#![feature(box_patterns)]

pub mod analysis;
pub mod utils;

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_metadata;
extern crate rustc_data_structures;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_hir;

use rustc_middle::ty::TyCtxt;
use rustc_driver::{Compilation, Callbacks};
use rustc_interface::{interface::Compiler, Queries};
use rustc_middle::util::Providers;
use rustc_interface::Config;
use rustc_session::search_paths::PathKind;
use rustc_data_structures::sync::Lrc;
use std::path::PathBuf;
use analysis::unsafety_isolation::UnsafetyIsolationCheck;
use analysis::callgraph::CallGraph;
use analysis::show_mir::ShowMir;

// Insert rustc arguments at the beginning of the argument list that RAP wants to be
// set per default, for maximal validation power.
pub static RAP_DEFAULT_ARGS: &[&str] =
    &["-Zalways-encode-mir", "-Zmir-opt-level=0", "--cfg=rap"];

pub type Elapsed = (i64, i64);

#[derive(Debug, Copy, Clone, Hash)]
pub struct RapCallback {
    upg: bool,
    unsafety_isolation: bool,
    unsafe_doc: bool,
    unsafe_cons: bool,
    callgraph: bool,
    show_mir: bool,
}

impl Default for RapCallback {
    fn default() -> Self {
        Self {
            upg: false,
            unsafety_isolation: false,
            unsafe_doc: false,
            unsafe_cons: false,
            callgraph: false,
            show_mir: false,
        }
    }
}


impl Callbacks for RapCallback {
    fn config(&mut self, config: &mut Config) {
        config.override_queries = Some(|_, providers| {
            providers.extern_queries.used_crate_source = |tcx, cnum| {
                let mut providers = Providers::default();
                rustc_metadata::provide(&mut providers);
                let mut crate_source = (providers.extern_queries.used_crate_source)(tcx, cnum);
                // HACK: rustc will emit "crate ... required to be available in rlib format, but
                // was not found in this form" errors once we use `tcx.dependency_formats()` if
                // there's no rlib provided, so setting a dummy path here to workaround those errors.
                Lrc::make_mut(&mut crate_source).rlib = Some((PathBuf::new(), PathKind::All));
                crate_source
            };
        });
    }

    fn after_analysis<'tcx>(
        &mut self,
        compiler: &Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        compiler.session().abort_if_errors();

        rap_info!("Execute after_analysis() of compiler callbacks");
        queries.global_ctxt().unwrap().enter(
            |tcx| start_analyzer(tcx, *self)
        );
        rap_info!("analysis done");

        compiler.session().abort_if_errors();
        Compilation::Continue
    }
}

impl RapCallback {
    pub fn enable_upg(&mut self) { 
	    self.upg = true; 
    }

    pub fn is_upg_enabled(&self) -> bool { 
	    self.upg 
    }

    pub fn enable_unsafety_isolation(&mut self) { 
        self.unsafety_isolation = true; 
    }
    
    pub fn is_unsafety_isolation_enabled(&self) -> bool { 
        self.unsafety_isolation 
    }

    pub fn enable_check_unsafe_doc(&mut self) { 
        self.unsafe_doc = true; 
    }

    pub fn is_check_unsafe_doc_enabled(&self) -> bool {
        self.unsafe_doc
    }

    pub fn enable_check_unsafe_cons(&mut self) { 
        self.unsafe_cons = true; 
    }

    pub fn is_check_unsafe_cons_enabled(&self) -> bool {
        self.unsafe_cons
    }

    pub fn enable_callgraph(&mut self) { 
	self.callgraph = true; 
    }

    pub fn is_callgraph_enabled(&self) -> bool { 
	self.callgraph 
    }

    pub fn enable_show_mir(&mut self) { 
	self.show_mir = true; 
    }

    pub fn is_show_mir_enabled(&self) -> bool { 
	self.show_mir 
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum RapPhase {
    Cleanup,
    Cargo,
    Rustc,
    LLVM, // unimplemented yet
}


pub fn compile_time_sysroot() -> Option<String> {
    // Optionally inspects an environment variable at compile time.
    if option_env!("RUSTC_STAGE").is_some() {
        // This is being built as part of rustc, and gets shipped with rustup.
        // We can rely on the sysroot computation in rustc.
        return None;
    }
    let home = option_env!("RUSTUP_HOME").or(option_env!("MULTIRUST_HOME"));
    let toolchain = option_env!("RUSTUP_TOOLCHAIN").or(option_env!("MULTIRUST_TOOLCHAIN"));
    let env = if home.is_some() && toolchain.is_some() {
         format!("{}/toolchains/{}", home.unwrap(), toolchain.unwrap())
    } else {
        option_env!("RUST_SYSROOT")
            .expect("To build RAP without rustup, set the `RUST_SYSROOT` env var at build time")
            .to_string()
    };
    Some(env)
}

pub fn start_analyzer(tcx: TyCtxt, callback: RapCallback) {
    if callback.is_upg_enabled() {
        UnsafetyIsolationCheck::new(tcx).start_generate_upg();
    }

    if callback.is_unsafety_isolation_enabled() {
        UnsafetyIsolationCheck::new(tcx).start_count_uig();
    }

    if callback.is_check_unsafe_doc_enabled() {
        UnsafetyIsolationCheck::new(tcx).start_check_doc();
    }

    if callback.is_check_unsafe_cons_enabled() {
        UnsafetyIsolationCheck::new(tcx).start_check_unsafe_cons();
    }

    if callback.is_callgraph_enabled() {
        CallGraph::new(tcx).start();
    }

    if callback.is_show_mir_enabled() {
        ShowMir::new(tcx).start();
    }
}

