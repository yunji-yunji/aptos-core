use std::fs;
// To test if my cm pass the bytecode-verifier
use crate::verifier::verify_module;

use move_core_types::identifier::{Identifier, is_valid};
use move_core_types::{
    account_address::AccountAddress,
    metadata::Metadata,
    vm_status::StatusCode,
};

use move_binary_format::CompiledModule;
use move_binary_format::internals::ModuleIndex;
use move_binary_format::file_format::{
    empty_module, /* basic_test_module, */
    AddressIdentifierIndex,
    FunctionInstantiation, FunctionHandle, FunctionHandleIndex, FunctionDefinition,
    FieldHandle, FieldHandleIndex, FieldInstantiation, FieldDefinition,
    IdentifierIndex,
    ModuleHandleIndex, ModuleHandle,
    SignatureIndex, Signature, SignatureToken,
    StructHandleIndex, StructHandle, StructFieldInformation,
    StructDefinitionIndex, StructDefinition, StructDefInstantiation,
    AbilitySet, Bytecode, CodeUnit, Constant, TypeSignature, Visibility, CompiledModuleField,
};

use rand::{Rng, SeedableRng, thread_rng};
use rand::prelude::Distribution;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::seq::IteratorRandom;
// use variant_count::VariantCount;

const P: bool= true;

fn mutate_vector<T>(vec:&mut Vec<T>, element: T) {
    // TODO: mutate order (if vector and if not empty and if more than 2)
    let mut rng = rand::thread_rng();
    let length: usize = vec.len();
    if length == 0 { // Empty // APPEND
        if P {println!("Op (empty)= Append")};
        vec.push(element);
    } else {
        let op = rng.gen_range(0..=3);
        let index = rng.gen_range(std::usize::MIN..length);
        if P {println!("Op SEED={}", op)};
        match op {
            0 => { // DELETE
                vec.remove(index);
            }, 
            1 => { // APPEND
                vec.push(element);
            },
            2 => { // INSERT
                vec.insert(index, element);
            },
            3 => { // MODIFY (Overwrite)
                vec[index] = element;
            },
            _ => {
                println!("Cannot find vector operation.")
            },
        }
    };
}

fn generate_valid_identifier(length: usize) -> Identifier {
    let mut rng = rand::thread_rng();
    let mut valid_str = String::with_capacity(length);

    while valid_str.len() < length {
        let next_char = match rng.gen_range(0..4) {
            0 => '_',
            1 => rng.gen_range(b'a'..=b'z') as char,
            2 => rng.gen_range(b'A'..=b'Z') as char,
            3 => rng.gen_range(b'0'..=b'9') as char,
            _ => '_',
            // _ => { println!("cannot generate random character");},
        };
        valid_str.push(next_char);
    }

    if is_valid(&valid_str) {
        Identifier::new(valid_str.to_string()).unwrap()
    } else {
        println!("Generated string is not valid = {:?}", valid_str);
        generate_valid_identifier(length)
        // return Identifier::new("<SELF>".to_string()).unwrap()
    }
}

#[derive(Debug, VariantCount, Copy)]
enum MutateOP {
    DELETE,
    INSERT,
    MIXORDER,
}

impl Distribution<MutateOP> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MutateOP {
        const ENUM_SIZE: usize = MutateOP::VARIANT_COUNT;
        // println!("enum size {:?}", ENUM_SIZE);
        match rng.gen_range(0..ENUM_SIZE) {
            0 => MutateOP::DELETE,
            1 => MutateOP::INSERT,
            2 => MutateOP::MIXORDER,
            _ => unreachable!(),
        }
    }
}

fn mutate_given_string(ident: Identifier) -> Identifier {
    let mut rng = rand::thread_rng();
    // all valid chracters
    // the byte value of the ASCII characters
    // let valid_chars: Vec<i32> = vec![97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57];
    let valid_chars: Vec<char> = vec!['_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    /*
    let lowercase_letters: Vec<u8> = (b'a'..=b'z').collect();
    let uppercase_letters: Vec<u8> = (b'A'..=b'Z').collect();
    let digits: Vec<u8> = (b'0'..=b'9').collect();
    let all_chars: Vec<u8> = lowercase_letters
        .iter()
        .chain(uppercase_letters.iter())
        .chain(digits.iter())
        .cloned()
        .collect();
    println!("{:?}", all_chars);
     */

    let old: String = ident.clone().into_string();
    let mut chars: Vec<char> = old.chars().collect();
    let len = old.len();
    
    let mut new: String = String::from("");

    match rng.gen_range(0..3) {
        0 => { // Delete some characters
            let num_to_delete = rng.gen_range(1..len);

            for _ in 0..num_to_delete {
                let idx = rng.gen_range(0..len);
                if idx < chars.len() {
                    chars.remove(idx);
                }
            }
            new = chars.into_iter().collect();
        },
        1 => { // Add some characters
            let num_to_select = rng.gen_range(1..len);
            let num_to_insert = rng.gen_range(1..=num_to_select);

            for _ in 0..num_to_insert {
                let idx = rng.gen_range(0..len);
                if let Some(&selected_char) = valid_chars.choose(&mut rand::thread_rng()) {
                    chars.insert(idx, selected_char);
                } else {
                    println!("FAIL TO CHOOSE");
                }
                // let selected_char = *rng.choose(&valid_chars).unwrap();
            }
            new = chars.into_iter().collect();

        }, 
        2 => { // mutate the order
            let len = chars.len();
            for i in (1..len).rev() {
                let j = rng.gen_range(0..=i);
                chars.swap(i, j);
            }
        
            new = chars.into_iter().collect();
        },
        _ => { println!("cannot mutate identifier");},
    };
    
    if is_valid(&new) {
        Identifier::new(new.to_string()).unwrap()
    } else {
        mutate_given_string(ident)
    }
}

pub fn mutate(module: &CompiledModule) -> CompiledModule {
    const VEC_LEN: usize= 4;
    const CODE_LEN: usize = 10;

    let mut m = module.clone(); // copy
    // const FIELD_SIZE: usize = CompiledModuleField::VARIANT_COUNT;
    // if P {println!("enum size = {:?}", FIELD_SIZE);};

    let mut rng = rand::thread_rng();
    let field_to_mutate: CompiledModuleField = rand::random();
    // let field_to_mutate = CompiledModuleField::IDENTIFIERS;
    // let field_to_mutate = CompiledModuleField::FUNCTION_DEFS;
    if P {println!("Field to mutate={:?}", field_to_mutate)};
    match field_to_mutate {
        CompiledModuleField::VERSION => {
            let rand_u32: u32 = rand::thread_rng().gen();
            m.version = rand_u32;
        }, 
        CompiledModuleField::SELF_MODULE_HANDLE_IDX => {
            // REMOVE?: mutation based on previous value
            // let prev_val :u16 =  m.self_module_handle_idx.into_index().try_into().unwrap();
            // let mut rng = StdRng::seed_from_u64(prev_val as u64);
            // m.self_module_handle_idx = ModuleHandleIndex(rng.gen_range(std::u16::MIN..=std::u16::MAX));

            m.self_module_handle_idx = rand::random();
        },
        CompiledModuleField::MODULE_HANDLES => {
            let new_module_handle: ModuleHandle = ModuleHandle {
                address: rand::random(),
                name: rand::random(),
            };
            mutate_vector(&mut m.module_handles, new_module_handle);
        },
        CompiledModuleField::STRUCT_HANDLES => {
            let new_struct_handle = StructHandle {
                module: rand::random(),
                name: rand::random(),
                abilities: rand::random(),
                type_parameters: vec![], // TODO
            };
            mutate_vector(&mut m.struct_handles, new_struct_handle);
        },
        CompiledModuleField::FUNCTION_HANDLES => {
            let new_function_handle = FunctionHandle {
                module: rand::random(),
                name: rand::random(),
                parameters: rand::random(),
                return_: rand::random(),
                type_parameters: vec![], // TODO
                access_specifiers: None, // TODO
            };
            mutate_vector(&mut m.function_handles, new_function_handle);
        },
        CompiledModuleField::FIELD_HANDLES => {
            let new_field_handle = FieldHandle {
                owner: rand::random(), // index
                field: rand::random(), // u16
            };
            mutate_vector(&mut m.field_handles, new_field_handle)
        },
        CompiledModuleField::FRIEND_DECLS => {
            let new_module_handle: ModuleHandle = ModuleHandle {
                address: rand::random(),
                name: rand::random(),
            };
            mutate_vector(&mut m.friend_decls, new_module_handle);
        },
        CompiledModuleField::STRUCT_DEF_INSTANTIATIONS => {
            let element = StructDefInstantiation {
                def: rand::random(),
                type_parameters: rand::random(),
            };
            mutate_vector(&mut m.struct_def_instantiations, element);
        },
        CompiledModuleField::FUNCTION_INSTANTIATIONS => {
            let element = FunctionInstantiation {
                handle: rand::random(),
                type_parameters: rand::random(),
            };
            mutate_vector(&mut m.function_instantiations, element);
        },
        CompiledModuleField::FIELD_INSTANTIATIONS => {
            let element = FieldInstantiation {
                handle: rand::random(),
                type_parameters: rand::random(),
            };
            mutate_vector(&mut m.field_instantiations, element);
        },
        CompiledModuleField::SIGNATURES => {
            let signature= rand::random();
            mutate_vector(&mut m.signatures, signature);
        },
        CompiledModuleField::IDENTIFIERS => {
            // 1. Mutate Existing Value
            for i in 0..m.identifiers.len() {
                let new = mutate_given_string(m.identifiers[i].clone());
                m.identifiers[i] = new;
            }

            // 2. Generate random string that is valid
            // (according to their valid check function)
            const STR_LEN: usize = 6;
            for _ in 0..rng.gen_range(0..VEC_LEN) {
                m.identifiers.push(generate_valid_identifier(STR_LEN));
            }
        },
        CompiledModuleField::ADDRESS_IDENTIFIERS => {
            // pub struct AccountAddress([u8; AccountAddress::LENGTH]);
            // this struct is fixed size vector
            // element type is u8.
            // length of the vector is AccountAddress::LENGTH
            let element = AccountAddress::random();
            mutate_vector(&mut m.address_identifiers, element);
        },
        CompiledModuleField::METADATA => {
            let element = rand::random();
            mutate_vector(&mut m.constant_pool, element);
        },
        CompiledModuleField::STRUCT_DEFS => {
            // TODO: length ???
            // TODO: like identifier? meaningful data?
            let length = rand::thread_rng().gen_range(1..=500);
            let random_k: Vec<u8> = (0..length).map(|_| rand::thread_rng().gen()).collect();
            let random_v: Vec<u8> = (0..length).map(|_| rand::thread_rng().gen()).collect();
            let element = Metadata {
                key: random_k,
                value: random_v,
            };
            mutate_vector(&mut m.metadata, element);
        },
        CompiledModuleField::STRUCT_DEFS => {
            let field_info = match rng.gen_range(0..=1) {
                0 => {
                    let mut field_defs: Vec<FieldDefinition> = vec!();
                    for _ in 0..rng.gen_range(0..VEC_LEN) {
                        mutate_vector(&mut field_defs, rand::random());
                    }
                    StructFieldInformation::Declared(field_defs)
                },
                1 => StructFieldInformation::Native,
                _ => StructFieldInformation::Native,
            };
            let element = StructDefinition {
                struct_handle: rand::random(),
                field_information: field_info,
            };
            mutate_vector(&mut m.struct_defs, element);
        },
        CompiledModuleField::FUNCTION_DEFS => {

            let mut acquires_global_resources_vec: Vec<StructDefinitionIndex> = vec![];
            for _ in 0..rng.gen_range(0..VEC_LEN) {
                mutate_vector(&mut acquires_global_resources_vec, rand::random());
            }

            // TODO: length of bytecodes who decide..
            // TODO: use prop_test
            let mut bytecodes: Vec<Bytecode> = vec!();
            for _ in 0..rng.gen_range(0..CODE_LEN) {
                mutate_vector(&mut bytecodes, rand::random());
            }

            let element = FunctionDefinition {
                function: rand::random(),
                visibility: rand::random(),
                is_entry: rand::random(), // boolean
                acquires_global_resources: acquires_global_resources_vec, // Vec<StructDefinitionIndex> 
                code: Some(CodeUnit {
                    locals: rand::random(),
                    code: bytecodes,
                }),
            };
            mutate_vector(&mut m.function_defs, element)
        },
        _ => (),
    }

    if P {println!("---------------------");};
    if P {println!("[Before mutate]"); module.print_field(field_to_mutate);};
    if P {println!("[After mutate]"); m.print_field(field_to_mutate.clone());};
    if P {println!("=====================");};

    m
}

#[test]
fn test_mutated_compiled_module() {
    // test 1
    let mut module: CompiledModule = empty_module();

    // test 2
    const NUM_OF_ITER: usize = 3;
    for _ in 0..NUM_OF_ITER {
        module = mutate(&module);
    }

    match verify_module(&module) {
        Ok(_) => (),
        Err(e) => {
            let status = e.major_status();
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