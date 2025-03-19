import? 'local.just'

default_args := 'build-reproducible-wasm'

counter := ''
factory_contract := "simple-factory-verify-rs-ci" + counter + ".testnet"

aux_counter := '-aux1'
child_deploy_signer := "child-deploy-signer" + aux_counter + ".testnet"
product_contract_name := "product"
product_from_factory_contract := product_contract_name + "." + factory_contract

product_standalone_contract := "standalone-simple-product-verify-rs-ci" + aux_counter + ".testnet"
factory_call_payload := "{ \"name\": \"" + product_contract_name + "\", \"beneficiary\": \"donatello2.testnet\"}"

[group('tempalte-create')]
_create_dev_acc target:
    near account create-account sponsor-by-faucet-service {{ target }} autogenerate-new-keypair save-to-keychain network-config testnet create || true

[group('create-acc')]
create_factory_dev_acc: (_create_dev_acc factory_contract)

[group('create-acc')]
create_child_deploy_signer_acc: (_create_dev_acc child_deploy_signer)

[group('create-acc')]
create_standalone_product_dev_acc: (_create_dev_acc product_standalone_contract)

[group('deploy')]
_deploy_generic folder target additional_args=default_args:
    cd {{ folder }} && cargo near deploy {{ additional_args }} {{ target }} without-init-call network-config testnet sign-with-keychain send

[group('deploy')]
deploy_factory_docker: create_factory_dev_acc (_deploy_generic "workspace_root_folder/factory" factory_contract)

[group('deploy')]
deploy_from_factory: create_child_deploy_signer_acc
    sleep 30
    near contract call-function as-transaction {{ factory_contract }} create_factory_subaccount_and_deploy json-args '{{ factory_call_payload }}' prepaid-gas '300.0 Tgas' attached-deposit '1.7 NEAR' sign-as {{ child_deploy_signer }} network-config testnet sign-with-keychain send

[group('deploy')]
deploy_product_standalone: create_standalone_product_dev_acc (_deploy_generic "workspace_root_folder/product-donation" product_standalone_contract)

[group('test-nep330-meta')]
_test_meta target:
    near contract call-function as-read-only {{ target }} contract_source_metadata json-args {} network-config testnet now

[group('test-nep330-meta')]
test_meta_factory: (_test_meta factory_contract)

[group('test-nep330-meta')]
test_meta_product: (_test_meta product_from_factory_contract)

[group('test-nep330-meta')]
test_meta_product_standalonw: (_test_meta product_standalone_contract)

[group('download-abi')]
_download_abi target:
    near contract download-abi {{ target }} save-to-file {{ target }}.json network-config testnet now

[group('download-wasm')]
_download_wasm target:
    near contract download-wasm {{ target }} save-to-file {{ target }}.wasm network-config testnet now

[group('download')]
_git_cleanup:
    git clean -f .

[group('download-all')]
show_wasm_hashes: && _git_cleanup
    #!/usr/bin/env zsh

    just _download_abi {{ factory_contract }}
    bat --paging never {{ factory_contract }}.json

    just _download_abi {{ product_from_factory_contract }}
    bat --paging never {{ product_from_factory_contract }}.json

    just _download_wasm {{ factory_contract }}
    just _download_wasm {{ product_from_factory_contract }}
    just _download_wasm {{ product_standalone_contract }}

    sha256sum *.wasm
