use actix_web::HttpResponse;
use aleo_rust::{
    AleoAPIClient, Encryptor, ProgramManager,
    snarkvm_types::{PrivateKey, Testnet3, Program, Process},
    AleoV0, Identifier, Locator, Query, BlockStore, BlockMemory
};
use serde_json::json;
use std::{str::FromStr, time::Instant};
use crate::helpers::{error, input};
use rand::thread_rng;

pub fn execute_credits_transfer_public(
    config: input::ProverConfig,
    payload: input::ExecutePayload
) -> Result<HttpResponse, error::InputError> {
    let total_now = Instant::now();
    let setup_now = Instant::now();
    let api_client = AleoAPIClient::<Testnet3>::testnet3();
    let private_key = PrivateKey::<Testnet3>::from_str(&config.private_key).unwrap();
    let private_key_ciphertext = Encryptor::<Testnet3>::encrypt_private_key_with_secret(
        &private_key, 
        &config.secret
    ).unwrap();

    let mut program_manager = ProgramManager::<Testnet3>::new(
        None, 
        Some(private_key_ciphertext), 
        Some(api_client), 
        None, 
        false).unwrap();

    // Calculating the time taken for the setup
    let setup_time = setup_now.elapsed();
    let execution_now = Instant::now();

    // execute program onchain
    let results = program_manager.execute_program(
        payload.program_id, 
        payload.function, 
        [payload.input_add, payload.input_amt].into_iter(), 
        0, 
        None, 
        Some(&config.secret), 
        None
    );

    match results {
        Ok(txhash) => {
            let execution_time = execution_now.elapsed();
            let total_time = total_now.elapsed();
            let response_body = json!({
                "txnHash": txhash,
                "total_time": total_time.as_millis().to_string() + "ms",
                "setup_time": setup_time.as_millis().to_string() + "ms",
                "execution_time": execution_time.as_millis().to_string() + "ms"
            });
            return Ok(HttpResponse::Ok().body(serde_json::to_string(&response_body).unwrap()));
        }
        Err(_) => {
            return Err(error::InputError::ExecutionFailed);
        }
    }
}

pub fn execute_offline_hello(
    config: input::ProverConfig,
    payload: input::ExecuteOfflinePayload
) -> Result<HttpResponse, error::InputError> {
    let total_now = Instant::now();
    let setup_now = Instant::now();
    let api_client = AleoAPIClient::<Testnet3>::testnet3();
    let private_key = PrivateKey::<Testnet3>::from_str(&config.private_key).unwrap();
    let private_key_ciphertext = Encryptor::<Testnet3>::encrypt_private_key_with_secret(
        &private_key, 
        &config.secret
    ).unwrap();

    let program_manager = ProgramManager::<Testnet3>::new(
        None, 
        Some(private_key_ciphertext), 
        Some(api_client), 
        None, 
        false).unwrap();

    // Defining a simple hello program with only a hello function
    let test_program = format!("program {};\n\nfunction hello:\n    input r0 as u32.public;\n    input r1 as u32.private;\n    add r0 r1 into r2;\n    output r2 as u32.private;\n", "hello.aleo");
    let program = Program::from_str(&test_program).unwrap();

    // Calculating the time taken for the setup
    let setup_time = setup_now.elapsed();
    let execution_now = Instant::now();

    // execute program offline
    let results = program_manager.execute_program_offline::<AleoV0>(
        &private_key,
        &program, 
        payload.clone().function_name, 
        &[], 
        [payload.clone().input_r0, payload.clone().input_r1].into_iter(), 
        true, 
        &config.query_url
    );

    match results {
        Ok(execution) => {
            let execution_time = execution_now.elapsed();
            let total_time = total_now.elapsed();
            let execution_result = json!({
                "id": execution.clone().execution_id().unwrap().to_string(),
                "proof": execution.clone().execution_proof().unwrap(),
                "total_time": total_time.as_millis().to_string() + "ms",
                "setup_time": setup_time.as_millis().to_string() + "ms",
                "execution_time": execution_time.as_millis().to_string() + "ms"
            });
            return Ok(HttpResponse::Ok().body(serde_json::to_string(&execution_result).unwrap()));
        }   
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(error::InputError::ExecutionFailed);
        }       
    }
}

pub fn prove_authorization(
    private_key: String
) -> Result<HttpResponse, error::InputError> {
    let total_now = Instant::now();
    let setup_now = Instant::now();
    let rng = &mut thread_rng();
    let pkey = PrivateKey::<Testnet3>::from_str(&private_key).unwrap();

    // Defining a simple hello program with only a hello function
    let test_program = format!("program {};\n\nfunction hello:\n    input r0 as u32.public;\n    input r1 as u32.private;\n    add r0 r1 into r2;\n    output r2 as u32.private;\n", "hello.aleo");
    let program = Program::from_str(&test_program).unwrap();

    // initializing a new process
    let mut process: Process<Testnet3> = Process::load().unwrap();
    process.add_program(&program).unwrap();

    // Check if program was added correctly
    let check_program = process.contains_program(program.id());
    assert!(check_program);

    let setup_time = setup_now.elapsed();
    let auth_now = Instant::now();
    let function = Identifier::<Testnet3>::try_from("hello").unwrap();
    let auth = process.authorize::<AleoV0,_>(
        &pkey, 
        program.id(), 
        function, 
        ["3u32", "5u32"].into_iter(), 
        rng
    ).unwrap();

    let auth_time = auth_now.elapsed();
    let execute_now = Instant::now();

    let (result, mut trace) = process.execute::<AleoV0,_>(auth, rng).unwrap();

    let execute_time = execute_now.elapsed();
    let prove_now = Instant::now();

    let locator = Locator::new(*program.id(), function);
    let block_store = BlockStore::<Testnet3, BlockMemory<_>>::open(None).unwrap();
    trace.prepare(Query::from(block_store)).unwrap();
    let prove_result = trace.prove_execution::<AleoV0,_>(&locator.to_string(), rng);

    match prove_result {
        Ok(prove) => {
            let prove_time = prove_now.elapsed();
            let total_time = total_now.elapsed();
            process.verify_execution(&prove).unwrap();
            let execution_result = json!({
                "id": prove.to_execution_id().unwrap(),
                "result": serde_json::to_string(&result.outputs().to_vec()).unwrap(),
                "proof": prove.proof().unwrap(),
                "verified": true,
                "auth_time": auth_time.as_millis().to_string() + "ms",
                "proof_time": prove_time.as_millis().to_string() + "ms",
                "total_time": total_time.as_millis().to_string() + "ms",
                "setup_time": setup_time.as_millis().to_string() + "ms",
                "execution_time": execute_time.as_millis().to_string() + "ms"
            });
            return Ok(HttpResponse::Ok().body(serde_json::to_string(&execution_result).unwrap()));
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(error::InputError::InvalidInputs);
        }
    }
}

pub fn execute_offline_multi(
    config: input::ProverConfig
) -> Result<HttpResponse, error::InputError> {
    let total_now = Instant::now();
    let setup_now = Instant::now();
    let api_client = AleoAPIClient::<Testnet3>::testnet3();
    let private_key = PrivateKey::<Testnet3>::from_str(&config.private_key).unwrap();
    let private_key_ciphertext = Encryptor::<Testnet3>::encrypt_private_key_with_secret(
        &private_key, 
        &config.secret
    ).unwrap();

    let program_manager = ProgramManager::<Testnet3>::new(
        None, 
        Some(private_key_ciphertext), 
        Some(api_client), 
        None, 
        false).unwrap();

    // Defining a simple hello program with only a hello function
    let test_program = format!("import helper.aleo;
    import fees.aleo;
    program {};
    
    record balance:
        owner as address.private;
        amount as u64.private;
    
    
    
    mapping account:
        key as address.public;
        value as u64.public;
    
    function transfer_public:
        input r0 as address.public;
        input r1 as u64.public;
        input r2 as u64.public;
        add r1 r2 into r3;
        call helper.aleo/mint_public self.caller r3 into r4 r5;
        call helper.aleo/verify_balance self.caller r3 into r6;
        call helper.aleo/transfer_relayer self.caller r2 into r7;
        call fees.aleo/fees 1u64 4475150356817606778589882704590642323349436680698264689983438181844155595696field into r8 r9;
        cast r0 r1 into r10 as balance.record;
        async transfer_public r5 r6 r7 r9 self.caller r0 r1 into r11;
        output r10 as balance.record;
        output r11 as multi_txn_t1.aleo/transfer_public.future;
    
    finalize transfer_public:
        input r0 as helper.aleo/mint_public.future;
        input r1 as helper.aleo/verify_balance.future;
        input r2 as helper.aleo/transfer_relayer.future;
        input r3 as fees.aleo/fees.future;
        input r4 as address.public;
        input r5 as address.public;
        input r6 as u64.public;
        await r0;
        await r1;
        await r2;
        await r3;
        get.or_use account[r4] 0u64 into r7;
        sub r7 r6 into r8;
        set r8 into account[r4];
        get.or_use account[r5] 0u64 into r9;
        add r9 r6 into r10;
        set r10 into account[r5];
    ", "multi_txn_t1.aleo");
    let program = Program::from_str(&test_program).unwrap();

    let im_1 = format!("program {};

    record balance:
        owner as address.private;
        amount as u64.private;
    
    
    mapping account:
        key as address.public;
        value as u64.public;
    
    function mint_public:
        input r0 as address.public;
        input r1 as u64.public;
        cast r0 r1 into r2 as balance.record;
        async mint_public r0 r1 into r3;
        output r2 as balance.record;
        output r3 as helper.aleo/mint_public.future;
    
    finalize mint_public:
        input r0 as address.public;
        input r1 as u64.public;
        get.or_use account[r0] r1 into r2;
        add r2 r1 into r3;
        set r3 into account[r0];
    
    
    function verify_balance:
        input r0 as address.public;
        input r1 as u64.public;
        async verify_balance r0 r1 into r2;
        output r2 as helper.aleo/verify_balance.future;
    
    finalize verify_balance:
        input r0 as address.public;
        input r1 as u64.public;
        get.or_use account[r0] 0u64 into r2;
        assert.neq r2 0u64;
    
    
    function transfer_relayer:
        input r0 as address.public;
        input r1 as u64.public;
        assert.eq r1 1u64;
        async transfer_relayer r0 aleo1va0hzrcsj569gz0gd0mvue7xk54vku626nsmvl86s7j490x7yupq89l82z r1 into r2;
        output r2 as helper.aleo/transfer_relayer.future;
    
    finalize transfer_relayer:
        input r0 as address.public;
        input r1 as address.public;
        input r2 as u64.public;
        get.or_use account[r0] 0u64 into r3;
        sub r3 r2 into r4;
        set r4 into account[r0];
        get.or_use account[r1] 0u64 into r5;
        add r5 r2 into r6;
        set r6 into account[r1];
    ", "helper.aleo");
    let im_program_1 = Program::from_str(&im_1).unwrap();

    let im_2 = format!("program {};

    record credits:
        owner as address.private;
        microcredits as u64.private;
    
    
    mapping account:
        key as address.public;
        value as u64.public;
    
    function fees:
        input r0 as u64.public;
        input r1 as field.public;
        assert.neq r0 0u64;
        assert.neq r1 0field;
        cast self.caller r0 into r2 as credits.record;
        async fees self.caller r0 into r3;
        output r2 as credits.record;
        output r3 as fees.aleo/fees.future;
    
    finalize fees:
        input r0 as address.public;
        input r1 as u64.public;
        get.or_use account[r0] 0u64 into r2;
        sub r2 r1 into r3;
        set r3 into account[r0];
    ", "fees.aleo");
    let im_program_2 = Program::from_str(&im_2).unwrap();

    // Calculating the time taken for the setup
    let setup_time = setup_now.elapsed();
    let execution_now = Instant::now();

    // execute program offline
    let results = program_manager.execute_program_offline::<AleoV0>(
        &private_key,
        &program, 
        "transfer_public", 
        &[im_program_1, im_program_2], 
        ["aleo1rn636g94mx3qqhf7m79nsne3llv4dqs25707yhwcrk92p0kwrc9qe392wg", "3u64", "1u64"].into_iter(), 
        true, 
        &config.query_url
    );

    match results {
        Ok(execution) => {
            let execution_time = execution_now.elapsed();
            let total_time = total_now.elapsed();
            let execution_result = json!({
                "id": execution.clone().execution_id().unwrap().to_string(),
                "proof": execution.clone().execution_proof().unwrap(),
                "total_time": total_time.as_millis().to_string() + "ms",
                "setup_time": setup_time.as_millis().to_string() + "ms",
                "execution_time": execution_time.as_millis().to_string() + "ms"
            });
            return Ok(HttpResponse::Ok().body(serde_json::to_string(&execution_result).unwrap()));
        }   
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(error::InputError::ExecutionFailed);
        }       
    }
}
