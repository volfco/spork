use std::path::PathBuf;
use deno_core::{op, Snapshot};
use deno_core::Extension;
use deno_core::{JsRuntime};
use deno_core::RuntimeOptions;

fn main()  {

    let ext = Extension::builder()
        .ops(vec![])
        .build();

    let state_file = PathBuf::from("state.bin");

    let mut will_snapshot = true;
    let startup_snapshot = if state_file.exists() {
        will_snapshot = false;
        let data = std::fs::read(&state_file).unwrap();
        Some(Snapshot::Boxed(data.into_boxed_slice()))
    } else {
        None
    };

    // Initialize a runtime instance
    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        startup_snapshot,
        will_snapshot,
        ..Default::default()
    });

    if !state_file.exists() {
        js_runtime.execute_script("test.js", include_str!("test.js")).unwrap();

        js_runtime.execute_script("inv", "it.next()").unwrap();
        let snapshot = js_runtime.snapshot();
        let snapshot_slice: &[u8] = &*snapshot;
        std::fs::write(&state_file, snapshot_slice).unwrap();
        println!("wrote state file");
    } else {
        js_runtime.execute_script("inv", "it.next()").unwrap();

    }





}
