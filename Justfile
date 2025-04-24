import? 'local.just'

funder_account := "cargo_near_test_workflows.testnet"
counter := "bind"

# devhub group

devhub_test_contract := "devhub-" + counter + ".testnet"
devhub_deposit := "'30 NEAR'"

# ====================
# community-factory group

community_factory_contract := "community" + "." + devhub_test_contract
community_factory_deposit := "'20 NEAR'"

# ====================
# community group

community_handle := 'uniquehandle'
community_contract := community_handle + "." + community_factory_contract

# ====================
# discussions grouop

discussions_contract := 'discussions' + "." + community_contract

# ====================
# payload

payload_create_community := '{ "inputs": { "handle": "' + community_handle + '", "name": "Gotham", "tag": "some", "description": "community.", "bio_markdown": "sample", "logo_url": "https://ipfs.near.social/ipfs/", "banner_url": "https://ipfs.near.social/" }}'

# ====================


[group('devhub')]
create_devhub_test_account:
    near account create-account fund-myself \
    {{ devhub_test_contract }} {{ devhub_deposit }} autogenerate-new-keypair \
    save-to-keychain sign-as {{ funder_account }} \
    network-config testnet sign-with-keychain send || true

[group('devhub')]
deploy_devhub: create_devhub_test_account
    cargo near deploy build-reproducible-wasm {{ devhub_test_contract }} with-init-call new \
    json-args {} prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' \
    network-config testnet sign-with-keychain send

[group('community-factory')]
create_community_factory_subaccount:
    near account create-account fund-myself {{ community_factory_contract }} \
        {{ community_factory_deposit }} autogenerate-new-keypair save-to-keychain \
        sign-as {{ devhub_test_contract }} network-config testnet \
        sign-with-keychain send

[group('community-factory')]
deploy_community_factory: create_community_factory_subaccount
    cd community-factory && cargo near deploy build-reproducible-wasm {{ community_factory_contract }} \
        without-init-call network-config testnet \
        sign-with-keychain send

[group('call-create-community')]
call_create_community_from_devhub:
    near contract call-function as-transaction {{ devhub_test_contract }} create_community \
         json-args '{{ payload_create_community }}' \
         prepaid-gas '300.0 Tgas' attached-deposit '6 NEAR' \
         sign-as {{ funder_account }} network-config testnet sign-with-keychain send

[group('template-deploy-from-faucet')]
_create_acc_and_deploy folder contract_account:
    near account create-account sponsor-by-faucet-service {{ contract_account }} autogenerate-new-keypair save-to-keychain network-config testnet create || true
    cd {{ folder }} && cargo near deploy build-reproducible-wasm {{ contract_account }} \
        without-init-call network-config testnet \
        sign-with-keychain send




## Testing

[group('account-summary')]
_view_summary target:
    near account view-account-summary {{ target }} network-config testnet now

[group('account-summary')]
view_summary_devhub: (_view_summary devhub_test_contract)

[group('account-summary')]
view_summary_community_factory: (_view_summary community_factory_contract)

[group('account-summary')]
view_summary_community: (_view_summary community_contract)


[group('account-summary')]
view_summary_discussions: (_view_summary discussions_contract)



[group('test-nep330-meta')]
_test_meta target:
    near --quiet contract call-function as-read-only {{ target }} contract_source_metadata json-args {} network-config testnet now

[group('test-nep330-meta')]
test_meta_devhub: (_test_meta devhub_test_contract)

[group('test-nep330-meta')]
test_meta_community_factory: (_test_meta community_factory_contract)

[group('test-nep330-meta')]
test_meta_community: (_test_meta community_contract)


[group('test-nep330-meta')]
test_meta_discussions: (_test_meta discussions_contract)



## Testing single command

[group('download-wasm')]
_download_wasm target:
    near contract download-wasm {{ target }} save-to-file {{ target }}.wasm network-config testnet now

[group('download-wasm')]
_git_cleanup:
    git clean -f .

[group('download-wasm')]
download_all_wasms: && _git_cleanup
    #!/usr/bin/env zsh
    just _download_wasm {{ devhub_test_contract }}
    just _download_wasm {{ community_factory_contract }}
    just _download_wasm {{ community_contract }}
    just _download_wasm {{ discussions_contract }}
    sha256sum *.wasm
