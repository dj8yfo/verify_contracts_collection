# verify_contracts_collection

- [x] `simple_package`
  - SHA-256 checksum bs58: 5KaX9FM9NtjpfahksL8TMWQk3LF7k8Sv88Qem4tGrVDW
  - nearblocks account link [simple-package-verify-rs-ci.testnet](https://testnet.nearblocks.io/address/simple-package-verify-rs-ci.testnet?tab=contract)
  - tag name: `simple-package-v1.0.0`
- [x] `simple-package-with-features`
  - SHA-256 checksum bs58: D5YfnZPCyzdqcdjroW7TGG3GQezdQSrcRWG4mRxdHx5d
  - nearblocks account link [simple-package-with-features-verify-rs-ci.testnet](https://testnet.nearblocks.io/address/simple-package-with-features-verify-rs-ci.testnet)
  - tag name: `simple-package-with-features-v1.0.0`
- [x] `simple-package-with-passed-env`
  - SHA-256 checksum bs58: 3fdG1ETP7SfArvdfeM9asqNfBj3HKvBK4ZV3uz3eTdzm
  - nearblocks account link [simple-package-with-paseed-env-verify-rs-ci.testnet](https://testnet.nearblocks.io/address/simple-package-with-paseed-env-verify-rs-ci.testnet?tab=contract)
    - expected source-scan verification ❌ **Code hash mismatch** (due to yet incorrect source-scan verification code)
  - tag name: `simple-package-with-passed-env-v1.0.0`
- [x] `simple-factory`
    - tag name: `simple-factory-v1.0.0+simple-factory-product-v1.0.0`
  - [x] `simple-factory`
    - SHA-256 checksum bs58: 3u4JhQTAzwMpwcmJ6NNVRJADqgxuN5HRuE9iiyGggNbK
    - nearblocks account link [simple-factory-verify-rs-ci.testnet](https://testnet.nearblocks.io/address/simple-factory-verify-rs-ci.testnet?tab=contract)
  - [x] `simple-factory-product`
    - (value from build log) Sub-build artifact SHA-256 checksum bs58: H3irQVgTVMYDPjyVsie25QqBrRzLJ3ff89dcFx9DMLFz
    - SHA-256 checksum bs58: H3irQVgTVMYDPjyVsie25QqBrRzLJ3ff89dcFx9DMLFz
    - nearblocks account link [product.simple-factory-verify-rs-ci.testnet](https://testnet.nearblocks.io/address/product.simple-factory-verify-rs-ci.testnet?tab=contract)
      - expected source-scan verification error ❌ **Error occurred during command...**
        (due to missing `[package.metadata.near.reproducible_build]` in manifest for `simple-factory-product` and yet incorrect source-scan verification code)
- [x] `simple-factory-with-features`
    - tag name: `simple-factory-with-features-v1.0.0+simple-factory-product-with-features-v1.0.0`
  - [x] `simple-factory-with-features`
    - SHA-256 checksum bs58: DRawD2V9JdcJTfuCvdqXQ4RzpP46wNeadi74P5C4cqaV
    - nearblocks account link [simple-factory-with-features-verify-rs-ci.testnet](https://testnet.nearblocks.io/address/simple-factory-with-features-verify-rs-ci.testnet?tab=contract)
  - [x] `simple-factory-product-with-features`
    - (value from build log) Sub-build artifact SHA-256 checksum bs58: 8XgSd1LzUJiouGz7nWN6cRfWj8Coyn36XUJFSkSDGptp
    - SHA-256 checksum bs58: 8XgSd1LzUJiouGz7nWN6cRfWj8Coyn36XUJFSkSDGptp
    - nearblocks account link [product.simple-factory-with-features-verify-rs-ci.testnet](https://testnet.nearblocks.io/address/product.simple-factory-with-features-verify-rs-ci.testnet?tab=contract)
      - expected source-scan verification error ❌ **Error occurred during command...**
        (due to missing `[package.metadata.near.reproducible_build]` in manifest for `simple-factory-product` and yet incorrect source-scan verification code)
