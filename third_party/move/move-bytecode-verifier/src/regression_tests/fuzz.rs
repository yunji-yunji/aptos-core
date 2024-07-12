use std::io::{self, Read,Write};
use std::fs::{self, File};

use move_binary_format::CompiledModule;
use move_core_types::vm_status::StatusCode;
use crate::verifier::verify_module;

#[test]
fn miri_path_fuzz() {

    // let module = file_format::empty_module();
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

fn write_cm_to_file(module: &CompiledModule, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = serde_cbor::to_vec(module)?;
    let mut file = File::create(file_path)?;
    file.write_all(&bytes)?;
    Ok(())
}

fn read_cm_from_file(file_path: &str) -> Result<CompiledModule, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let module: CompiledModule = serde_cbor::from_slice(&bytes)?;

    Ok(module)
}

fn read_cm_stdin() -> Result<CompiledModule, Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    io::stdin().read_to_end(&mut bytes)?;
    let module: CompiledModule = serde_cbor::from_slice(&bytes)?;
    Ok(module)
}
