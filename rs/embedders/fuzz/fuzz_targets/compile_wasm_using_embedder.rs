#![no_main]
use ic_config::embedders::Config as EmbeddersConfig;
use ic_embedders::{wasm_utils::compile, WasmtimeEmbedder};
use ic_logger::replica_logger::no_op_logger;
use ic_wasm_types::BinaryEncodedWasm;
use libfuzzer_sys::fuzz_target;
use wasm_smith::MaybeInvalidModule;

// This fuzz test tries to fuzz wasm compilation. The idea is not so much to crash the wasm compilation
// but get wasm files that could take long compilation times (>10 seconds). This can be monitored running
// libfuzzer with a specific timeout, in Clusterfuzz this can be achieved using 'TEST_TIMEOUT' environment
// string from the template.
//
// Given that WASM binaries with a size of up to 30MB are expected for this feature, the payload size
// (i.e., max_len parameter in libfuzzer) needs to be also ajusted.
//
// The fuzz test is only compiled but not executed by CI.
//
// To execute the fuzzer run
// bazel run --config=fuzzing //rs/embedders/fuzz:compile_wasm_using_embedder -- corpus/

fuzz_target!(|module: MaybeInvalidModule| {
    let config = EmbeddersConfig::default();
    let wasm = module.to_bytes();
    let binary_wasm = BinaryEncodedWasm::new(wasm);
    let embedder = WasmtimeEmbedder::new(config, no_op_logger());

    let (_, _) = compile(&embedder, &binary_wasm);
});
