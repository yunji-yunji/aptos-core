use std::io::{self, Read,Write};
use std::fs::{self, File};

use move_binary_format::file_format::{self, basic_test_module, empty_module};
use move_binary_format::CompiledModule;
use move_core_types::vm_status::StatusCode;
use crate::verifier::verify_module;

#[test]
fn miri_path_fuzz_stdin() {
    let read_module = read_cm_stdin();
    let module = match read_module {
        Ok(m) => m,
        Err(_) => { panic!("cannot read module."); },
    };
    println!("run miri path fuzz CompiledModule: {:?}", module.version);

    match verify_module(&module) {
        Ok(_) => (),
        Err(e) => {
            let status = e.major_status();
            println!("verify module failed! {:?}", status);

            // additionally force a panic on status code that should not been reached
            match status {
                StatusCode::UNKNOWN_VALIDATION_STATUS => unreachable!("UNKNOWN_VALIDATION_STATUS"),
                StatusCode::UNKNOWN_VERIFICATION_ERROR => unreachable!("UNKNOWN_VERIFICATION_ERROR"),
                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR => unreachable!("UNKNOWN_INVARIANT_VIOLATION_ERROR"),
                StatusCode::UNREACHABLE => unreachable!("UNREACHABLE"),
                StatusCode::UNEXPECTED_ERROR_FROM_KNOWN_MOVE_FUNCTION => unreachable!("UNEXPECTED_ERROR_FROM_KNOWN_MOVE_FUNCTION"),
                StatusCode::VERIFIER_INVARIANT_VIOLATION => unreachable!("VERIFIER_INVARIANT_VIOLATION"),
                StatusCode::UNEXPECTED_VERIFIER_ERROR => unreachable!("UNEXPECTED_VERIFIER_ERROR"),
                StatusCode::UNEXPECTED_DESERIALIZATION_ERROR => unreachable!("UNEXPECTED_DESERIALIZATION_ERROR"),
                _ => (),
            }
        }
    }
}

// cargo test --package move-bytecode-verifier --lib -- regression_tests::fuzz::miri_path_fuzz --exact --show-output data=""
#[test]
fn miri_path_fuzz() {
    let args: Vec<String> = std::env::args().collect();
    let mut data_arg: Option<String> = None;
    for arg in args.iter().skip(1) {
        if arg.starts_with("data=") {
            data_arg = Some(arg.chars().skip(5).collect());
            break;
        }
    }
    if let Some(data_raw) = data_arg {
        println!("\n- file name: {:?}", data_raw);

        let read_module = read_cm_from_file(&data_raw);
        let module = match read_module {
            Ok(m) => m,
            Err(_) => {
                // panic!("cannot read module.");
                println!("[info] cannot read module. check the input file: {:?}", data_raw);
                let mut file = File::open(data_raw).expect("open file");
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes).unwrap();
                println!("[info] bytes: {:?}", bytes);
                empty_module()
            },
        };
        println!("CompiledModule: {:?}", module);

        // let module = basic_test_module();

        let res = verify_module(&module);
        println!("verify_module result: {:?}", res);
    }
}

fn read_cm_from_file(file_path: &str) -> Result<CompiledModule, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let module: CompiledModule = serde_cbor::from_slice(&bytes)?;
    // let module: CompiledModule = serde_json::from_slice(&bytes).expect("failed to read (serde_json)");

    Ok(module)
}

fn read_cm_stdin() -> Result<CompiledModule, Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    io::stdin().read_to_end(&mut bytes)?;
    let module: CompiledModule = serde_cbor::from_slice(&bytes).expect("failed to deserialize CompiledModule");
    // let module: CompiledModule = serde_json::from_slice(&bytes).expect("failed to read (serde_json)");

    Ok(module)
}

// *** For debugging *** //
fn write_cm_to_file(module: &CompiledModule, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = serde_cbor::to_vec(module)?;
    let mut file = File::create(file_path)?;
    file.write_all(&bytes)?;
    Ok(())
}

// cargo test --package move-bytecode-verifier --lib -- regression_tests::fuzz::generate_test_module --exact --show-output
#[test]
fn generate_test_module() {
    let test_cm = empty_module();
    // let test_cm = basic_test_module();
    write_cm_to_file(&test_cm, "src/regression_tests/empty_cm").unwrap();
}