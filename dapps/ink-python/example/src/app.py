# https://polkascan.github.io/py-substrate-interface/usage
# https://github.com/polkascan/py-substrate-interface
import os
from dotenv import load_dotenv
from substrateinterface import SubstrateInterface, Keypair, KeypairType, ContractCode, ContractInstance

load_dotenv()

LS_CONTRACTS = os.getenv('LS_CONTRACTS')

provider_rococo = "wss://rococo-contracts-rpc.polkadot.io"
# provider_local = "ws://127.0.0.1:9944"
substrate = SubstrateInterface(
    url=provider_rococo,
    ss58_format=42,
    type_registry_preset='substrate-node-template',
)

# default development mnemonic
keypair_alice = Keypair.create_from_uri(
    "//Alice",
    crypto_type=KeypairType.SR25519
)
print(keypair_alice.ss58_address)

keypair_bob = Keypair.create_from_uri(
    "//Bob",
    crypto_type=KeypairType.SR25519
)
print(keypair_bob.ss58_address)

keypair_charlie = Keypair.create_from_uri(
    "//Charlie",
    crypto_type=KeypairType.SR25519
)
print(keypair_charlie.ss58_address)
# FIXME - conditionally run only if `LS_CONTRACTS` is defined
# FIXME - why does this return `ValueError: Invalid mnemonic: invalid word in phrase`
keypair_ls = Keypair.create_from_mnemonic(
    LS_CONTRACTS,
    crypto_type=KeypairType.SR25519
)
print(keypair_ls.ss58_address)

# deploy contract
# https://polkascan.github.io/py-substrate-interface/usage/ink-contract-interfacing/#work-with-an-existing-instance
# https://stackoverflow.com/questions/2860153/how-do-i-get-the-parent-directory-in-python
up = [os.pardir]*4
go_up = os.path.join(*up)
code = ContractCode.create_from_contract_files(
    metadata_file=os.path.join(os.path.dirname(__file__), go_up, 'target', 'ink', 'flipper', 'flipper.json'),
    wasm_file=os.path.join(os.path.dirname(__file__), go_up, 'target', 'ink', 'flipper', 'flipper.wasm'),
    substrate=substrate
)
contract = code.deploy(
    keypair=keypair_ls,
    constructor="new",
    args={'init_value': True},
    value=0,
    # endowment=0,
    gas_limit={'ref_time': 25990000000, 'proof_size': 11990}, # gas_limit=1000000000000, # gas_limit: dict = None,
    deployment_salt="",
    upload_code=True,
    storage_deposit_limit=1000000000000
)
print(f'✅ Deployed @ {contract.contract_address}')

# Check if contract is on chain
# obtain contract address hex and paste at
# https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-contracts-rpc.polkadot.io#/chainstate
# contracts > ownerInfoOf
contract_info = substrate.query("Contracts", "ContractInfoOf", [contract.contract_address])
print(f'Existing deployed @ {contract_info}')
# create contract instance from deterministic address
contract_existing = ContractInstance.create_from_address(
    contract_address=contract.contract_address,
    metadata_file=os.path.join(os.path.dirname(__file__), go_up, 'target', 'ink', 'flipper', 'flipper.json'),
    substrate=substrate
)
print(f'Existing deployed @ {contract_existing}')
print(f'Existing deployed @ {contract_existing.contract_address}')

# read data from contract
result = contract.read(keypair_ls, 'get')
print('Current value of "get":', result.contract_result_data)

# read data from contract
result = contract_existing.read(keypair_ls, 'get')
print('Current value of "get":', result.contract_result_data)

# keypair creation and signing
# mnemonic = Keypair.generate_mnemonic()
# keypair = Keypair.create_from_mnemonic(
#     mnemonic,
#     crypto_type=KeypairType.SR25519 # or ECDSA, ED25519 
# )
# print(keypair.ss58_address)
# signature = keypair.sign("Test123")
# if keypair.verify("Test123", signature):
#     print('Verified')

# # generate signature payload then use to perform offline signing of extrinsics
# # on another offline machine and later send to network with generated signature
# call = substrate.compose_call(
#     call_module='Balances',
#     call_function='transfer',
#     call_params={
#         'dest': keypair_bob.ss58_address,
#         'value': 2 * 10**8
#     }
# )
# era = {'period': 64, 'current': 22719}
# nonce = 0
# signature_payload = substrate.generate_signature_payload(call=call, era=era, nonce=nonce)
# # generate the signature with given `signature_payload` on another offline machine
# # TODO - find mnemonic phrase of Charlie and replace line below
# keypair = Keypair.create_from_mnemonic("nature exchange gasp toy result bacon coin broccoli rule oyster believe lyrics")
# signature = keypair.sign(signature_payload)
# # finally on online machine send the extrinsic with generated signature
# keypair = Keypair(ss58_address=keypair_alice.ss58_address)
# extrinsic = substrate.create_signed_extrinsic(
#     call=call,
#     keypair=keypair,
#     era=era,
#     nonce=nonce,
#     signature=signature
# )
# result = substrate.submit_extrinsic(
#     extrinsic=extrinsic
# )
# print(result.extrinsic_hash)

# # query single for balance of an account
# result = substrate.query('System', 'Account', ['F4xQKRUagnSGjFqafyhajLs94e7Vvzvr8ebwYJceKpr8R7T'])
# print(result.value['data']['free']) # 635278638077956496
# print(result.value['nonce']) #  7695

# # query multiple storage entries and batch them in a single RPC request for account info
# storage_keys = [
#     substrate.create_storage_key(
#         "System", "Account", ["F4xQKRUagnSGjFqafyhajLs94e7Vvzvr8ebwYJceKpr8R7T"]
#     ),
#     substrate.create_storage_key(
#         "System", "Account", ["GSEX8kR4Kz5UZGhvRUCJG93D5hhTAoVZ5tAe6Zne7V42DSi"]
#     ),
#     substrate.create_storage_key(
#         "Staking", "Bonded", ["GSEX8kR4Kz5UZGhvRUCJG93D5hhTAoVZ5tAe6Zne7V42DSi"]
#     )
# ]
# result = substrate.query_multi(storage_keys)
# for storage_key, value_obj in result:
#     print(storage_key, value_obj)

# # state including balance at specific block hash
# account_info = substrate.query(
#     module='System',
#     storage_function='Account',
#     params=['F4xQKRUagnSGjFqafyhajLs94e7Vvzvr8ebwYJceKpr8R7T'],
#     block_hash='0x176e064454388fd78941a0bace38db424e71db9d5d5ed0272ead7003a02234fa'
# )
# print(account_info['nonce'].value) #  7673
# print(account_info['data']['free'].value) # 637747267365404068

# # query mapped storage function
# # example: retrieve the first 199 System.Account entries
# result = substrate.query_map('System', 'Account', max_results=199)
# for account, account_info in result:
#     print(f"Free balance of account '{account.value}': {account_info.value['data']['free']}")

# # retrieve all System.Account entries in batches of 200 (automatically appended by `QueryMapResult` iterator)
# # note: batches retrieved are capped by `page_size`, and the max restricted by an RPC node is 1000
# result = substrate.query_map('System', 'Account', page_size=200, max_results=400)
# for account, account_info in result:
#     print(f"free balance of account '{account.value}': {account_info.value['data']['free']}")

# # query `DoubleMap` storage function
# era_stakers = substrate.query_map(
#     module='Staking',
#     storage_function='ErasStakers',
#     params=[2100]
# )

# # type decomposition to retrieve and format storage function parameters
# storage_function = substrate.get_metadata_storage_function("Tokens", "TotalIssuance")
# print(storage_function.get_param_info())
# # [{
# #   'Token': ('ACA', 'DOT', 'KAR', 'KSM', 'BNC', ),  
# #   'Erc20': '[u8; 20]', 
# #   'StableAssetPoolToken': 'u32', 
# #   'ForeignAsset': 'u16'
# # }]

# # nested Struct is a ScaleType object
# account_info = <AccountInfo(
#     value={
#         'nonce': <U32(value=5)>,
#         'consumers': <U32(value=0)>,
#         'providers': <U32(value=1)>,
#         'sufficients': <U32(value=0)>,
#         'data': <AccountData(
#             value={'free': 1152921503981846391,
#                    'reserved': 0,
#                    'misc_frozen': 0,
#                    'fee_frozen': 0
#             }
#         )>
#     }
# )>
# # access nested structure
# account_info['data']['free']
# # convert to an iterable
# for other_info in era_stakers['others']:
#     print(other_info['who'], other_info['value'])
# print(account_info.serialize())

# # compare the internally serialized `value` attribute of ScaleType objects with python primitives
# # metadata_obj[1][1]['extrinsic']['version'] # '<U8(value=4)>'
# # metadata_obj[1][1]['extrinsic']['version'] == 4 # True

# # storage query subscription with updates pushed to the callable `subscription_handler` and will
# # block execution until a final value is returned and finally automatically unsubscribed from
# # further updates
# def subscription_handler(account_info_obj, update_nr, subscription_id):
#     if update_nr == 0:
#         print('Initial account data:', account_info_obj.value)
#     if update_nr > 0:
#         # Do something with the update
#         print('Account data changed:', account_info_obj.value)
#     # The execution will block until an arbitrary value is returned, which will be the result of the `query`
#     if update_nr > 5:
#         return account_info_obj
# result = substrate.query("System", "Account", ["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"],
#                          subscription_handler=subscription_handler)
# print(result)

# # storage query subscription to multiple keys at once. tracks changes for multiple
# # state entries (storage keys) in a single RPC call to a Substrate node with updates
# # pushed to the `subscription_handler` callable and will block execution until a final
# # value is returned as a result of the subscription and finally unsubscribed from further updates 
# def subscription_handler(storage_key, updated_obj, update_nr, subscription_id):
#     print(f"Update for {storage_key.params[0]}: {updated_obj.value}")
# # Accounts to track
# storage_keys = [
#     substrate.create_storage_key(
#         "System", "Account", ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
#     ),
#     substrate.create_storage_key(
#         "System", "Account", ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"]
#     )
# ]
# result = substrate.subscribe_storage(
#     storage_keys=storage_keys, subscription_handler=subscription_handler
# )

# # storage query subscription to new block headers
# def subscription_handler(obj, update_nr, subscription_id):
#     print(f"New block #{obj['header']['number']}")
#     block = substrate.get_block(block_number=obj['header']['number'])
#     for idx, extrinsic in enumerate(block['extrinsics']):
#         print(f'# {idx}:  {extrinsic.value}')
#     if update_nr > 10:
#         return {'message': 'Subscription will cancel when a value is returned', 'updates_processed': update_nr}

# result = substrate.subscribe_block_headers(subscription_handler)