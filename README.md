# Avail Prover Demo

## Environment variables
Add the following details to your `config.json` file. Use the aleo sdk to obtain your account private key. The secret is used to encrypt your private key.

```
{
    "private_key": "APrivateKey...",
    "query_url": "https://api.explorer.aleo.org/v1",
    "secret": "***"
}
```

## Instructions
To start the prover, use `cargo run`

## Api calls for the prover

### 1. Fetch program 

```cmd
curl --location --request GET 'http://localhost:3030/welcome'
```

#### Expected output: 
Fetches the `credits.aleo` program from the aleo networks and prints out the program on the console.

```
{
    "total_time":"972ms",
    "setup_time":"0ms",
    "execution_time":"971ms"
}
```

### 2. Execute offline [Simple hello.aleo program]
```cmd
curl --location --request POST 'http://localhost:3030/executeOffline' \
--header 'Content-Type: application/json' \
--data-raw '{"function_name":"hello",
"input_r0":"5u32",
"input_r1":"3u32"
}'
```

#### Expected output: 
The execution id and the execution proof are received as response.

```
{
    "id":"4191400112758423097505179406038622766609655052058057409092812982026189670291field",
    "proof":"proof1qyqsqqqqqqqqqqqpqqqqqqqqqqqpxah3qn7lu7vvem423y0z5akvmv2l78yrzgvgdw33f28ms388v28kczp74alp9pdjlsskfaz55fuqqy8swrkarpzyfeued09evdmlck704uyrhhs03pguz3d6u4e09qt8plsdazd8d40hjg4wy82ee45nrqg0snuqest0u6qx86trz7h6wvdx350hxxp2e4kx68tar4cylwqvudfd8ux270yevjzpv4dz42ah9jqzwpd40ufjtap97gya6lv8csu583zakp3wkjezemma98ewuc24m80kphd02u8n52pv2avhf820lyupc5087vvga9d7sra3k59fxfkgprtgtwuuf07eujeq4pfn25f6tyzdsv59ya9n099zke2txdtwjgeqzf2mtq4txz7wfk37h8we4q9judunr738z846nyv0nekhw07nxxc429pe4frdcr3g5rs7c8xpjd4xqp0chmjtfd0qh9zqqn53pjslcflnce0y24dgnzmd0rhr2yywg4gp7czdpsmgr4tlcrt5tqkrltv92qt9r5hm46lpt5a962m07379e2vhy6lu7rjge3t8wyhgjqhhc4z5evd7esllkncth5sxs877jec8m7qt7a69z5cadm2mdem9wz2zrqg99g926u8xge2gcyygdzye6m9rm0xmqmr5kem2ahxfl6atxethtfuqwmlz2pzze97zyeze2p9nt6whapfd9jzpkjce3xwpz0cl8wqxwugkeu5eh8vc9a4jvkk0vqvj4lfx6zu8wlf9cc306de44h7fefaxwy4pnf567534thup7wdwqkrnexph3wxtsaehtu2zp05ghavlvpauzys4vtm59wfedu2zqpax8ruqu0ltcuuxkkd94scfmaky08jyvh5sc4d3kgy0v5uegfdhxu8zaca98qu5vk2c4g8l8rawraaexw5w8kc3c4uqltnmpc54m3x7mwhp8h8dl2yrz9c9ttz60rq7q2z9grplxygygqlss86guq3wtpqqpggeweaqqk4rzph4meh3k9dkevc0q6pf5znm2ksh99fccgz0qr2lumj2j9hjyk0ug0yyfnea38mks27zclkrpynczz8gu870se0yl6365plrr7cn9u8lfp698egjnfxxfsn9w70sy823fhvfp89fe8hkprsgfl26lludtk480pvjdhzd3pl9hl8jnzqyqvqqqqqqqqqqpu2tue69ykflltkf7krdr5z8qsc0egpp7jvfrcn62r2sakxp06vw87nef26pzg2w53hv4se6ejhcqqqz2fhtlvk0lz0huqvpl07h2j02s9ek5aelsfhtnzrpzeh5vlh2fx9hcsgyk5pgza2kmyhzh2j83uyqqxejc4c28fkzkccfn233gxjxakaxq3ulhfm762nsqwjjjjc3ujjpr9vlwt9hps9svhs57luc0q5w88xgxymkhx8azf5uajj9j85dt7dxjphzjhlgkdl6m7ngeytvm9t6qyqq7syh2q",
    "total_time":"79460ms",
    "setup_time":"40ms",
    "execution_time":"79419ms"
}
```

#### Printed on server:
```
Checking function hello exists in hello.aleo
✅ Request              (Constant: 24561, Public: 9, Private: 11329, Constraints: 11334, NonZeros: (25769, 35398, 15350))
✅ Function 'hello()'   (Constant: 24561, Public: 9, Private: 11361, Constraints: 11367, NonZeros: (25897, 35431, 15382))
✅ Response             (Constant: 25169, Public: 10, Private: 12646, Constraints: 12653, NonZeros: (31196, 44743, 16676))
✅ Complete             (Constant: 25169, Public: 10, Private: 12646, Constraints: 12653, NonZeros: (31196, 44743, 16676))
✅ Request              (Constant: 3121, Public: 9, Private: 11329, Constraints: 11334, NonZeros: (25769, 35398, 15350))
✅ Function 'hello()'   (Constant: 3121, Public: 9, Private: 11361, Constraints: 11367, NonZeros: (25897, 35431, 15382))
✅ Response             (Constant: 3729, Public: 10, Private: 12646, Constraints: 12653, NonZeros: (31196, 44743, 16676))
✅ Complete             (Constant: 3729, Public: 10, Private: 12646, Constraints: 12653, NonZeros: (31196, 44743, 16676))
Number of padded public variables: 16
Number of private variables: 12649
Number of num_constraints: 12654
Number of non-zero entries in A: 31197
Number of non-zero entries in B: 44744
Number of non-zero entries in C: 16677
Loading 65536 powers
Loading 65536 shifted powers
Number of padded public variables in Prover::Init: 16
Number of private variables: 12649
Number of constraints: 12654
Number of non-zero entries in A: 31197
Number of non-zero entries in B: 44744
Number of non-zero entries in C: 16677
```

### 3. Execute [transfer_public function, credits.aleo program]
```cmd
curl --location --request POST 'http://localhost:3030/execute' \
--header 'Content-Type: application/json' \
--data-raw '{"program_id":"credits.aleo",
"function":"transfer_public",
"input_add":"aleo1va0hzrcsj569gz0gd0mvue7xk54vku626nsmvl86s7j490x7yupq89l82z",
"input_amt":"3u64"
}'
```

#### Expected output: 
The transaction hash is returned as response.
```
{
    "txnHash":"\"at1tk582fsea9x6kyhxajtm4zx98f64ld78h2se8jrgq0fw4j5x55psz4mm9p\"",
    "total_time":"171023ms",
    "setup_time":"11ms",
    "execution_time":"171012ms"
}
```

#### Printed on server:
```
Checking function transfer_public exists in credits.aleo
✅ Request              (Constant: 24914, Public: 9, Private: 11048, Constraints: 11054, NonZeros: (23717, 30908, 15237))
✅ Function 'transfer_public()' (Constant: 25122, Public: 9, Private: 11048, Constraints: 11054, NonZeros: (23717, 30908, 15237))
✅ Response             (Constant: 25944, Public: 10, Private: 12043, Constraints: 12052, NonZeros: (27250, 36303, 16407))
✅ Complete             (Constant: 25944, Public: 10, Private: 12043, Constraints: 12052, NonZeros: (27250, 36303, 16407))
✅ Request              (Constant: 3475, Public: 9, Private: 11048, Constraints: 11054, NonZeros: (23717, 30908, 15237))
✅ Function 'transfer_public()' (Constant: 3683, Public: 9, Private: 11048, Constraints: 11054, NonZeros: (23717, 30908, 15237))
✅ Response             (Constant: 4505, Public: 10, Private: 12043, Constraints: 12052, NonZeros: (27250, 36303, 16407))
✅ Complete             (Constant: 4505, Public: 10, Private: 12043, Constraints: 12052, NonZeros: (27250, 36303, 16407))
Number of padded public variables: 16
Number of private variables: 12046
Number of num_constraints: 12053
Number of non-zero entries in A: 27251
Number of non-zero entries in B: 36304
Number of non-zero entries in C: 16408
Loading 65536 powers
Loading 65536 shifted powers
Number of padded public variables in Prover::Init: 16
Number of private variables: 12046
Number of constraints: 12053
Number of non-zero entries in A: 27251
Number of non-zero entries in B: 36304
Number of non-zero entries in C: 16408
✅ Request              (Constant: 3353, Public: 10, Private: 11582, Constraints: 11588, NonZeros: (25926, 34623, 15703))
✅ Function 'fee_public()' (Constant: 3586, Public: 10, Private: 11650, Constraints: 11659, NonZeros: (26314, 34696, 15771))
✅ Response             (Constant: 4364, Public: 11, Private: 12645, Constraints: 12657, NonZeros: (29594, 39585, 16941))
✅ Complete             (Constant: 4364, Public: 11, Private: 12645, Constraints: 12657, NonZeros: (29594, 39585, 16941))
✅ Request              (Constant: 3353, Public: 10, Private: 11582, Constraints: 11588, NonZeros: (25926, 34623, 15703))
✅ Function 'fee_public()' (Constant: 3586, Public: 10, Private: 11650, Constraints: 11659, NonZeros: (26314, 34696, 15771))
✅ Response             (Constant: 4364, Public: 11, Private: 12645, Constraints: 12657, NonZeros: (29594, 39585, 16941))
✅ Complete             (Constant: 4364, Public: 11, Private: 12645, Constraints: 12657, NonZeros: (29594, 39585, 16941))
Number of padded public variables: 16
Number of private variables: 12648
Number of num_constraints: 12658
Number of non-zero entries in A: 29595
Number of non-zero entries in B: 39586
Number of non-zero entries in C: 16942
Number of padded public variables in Prover::Init: 16
Number of private variables: 12648
Number of constraints: 12658
Number of non-zero entries in A: 29595
Number of non-zero entries in B: 39586
Number of non-zero entries in C: 16942
Attempting to broadcast execution transaction for credits.aleo
✅ Execute Transaction successfully posted to https://api.explorer.aleo.org/v1
✅ Execution of function "transfer_public" from program credits.aleo' broadcast successfully
```
