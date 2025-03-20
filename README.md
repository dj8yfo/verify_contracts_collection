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
    - tag name: `simple-factory-v1.0.0+simple-factory-product-v1.1.0`
  - [x] `simple-factory-v1.0.0`
    - SHA-256 checksum bs58: 7qhDddxfr4p39CeBvpTXWQmzzDA4HTbrWceZtaDAExjW
    - nearblocks account link [simple-factory-verify-rs-cia.testnet](https://testnet.nearblocks.io/address/simple-factory-verify-rs-cia.testnet?tab=contract)
  - [x] `simple-factory-product-v1.1.0`
    - (value from build log) Sub-build artifact SHA-256 checksum bs58: FLXsv6msJ6dD9A6DpJX96d3q8UiDjUtyBsdnEYVnML2U
    - SHA-256 checksum bs58: FLXsv6msJ6dD9A6DpJX96d3q8UiDjUtyBsdnEYVnML2U
    - nearblocks account link [product.simple-factory-verify-rs-cia.testnet](https://testnet.nearblocks.io/address/product.simple-factory-verify-rs-cia.testnet?tab=contract)
      - expected source-scan verification error ❌ **Error occurred during command...**
        (due to missing `[package.metadata.near.reproducible_build]` in manifest for `simple-factory-product` and yet incorrect source-scan verification code)
- [x] `simple-factory-with-features`
    - tag name: `simple-factory-with-features-v1.0.0+simple-factory-product-with-features-v1.1.0`
  - [x] `simple-factory-with-features`
    - SHA-256 checksum bs58: 6Nmb4WML7VpKmv8KCJzxMD6SQ1jjhwiVRbKYkx2Jqts1
    - nearblocks account link [simple-factory-with-features-verify-rs-ci-a.testnet](https://testnet.nearblocks.io/address/simple-factory-with-features-verify-rs-ci-a.testnet?tab=contract)
  - [x] `simple-factory-product-with-features`
    - (value from build log) Sub-build artifact SHA-256 checksum bs58: 2onZk3T9QqqNTEMwHf6EGBtLUEa4WyebtxDfYzhq5mLW
    - SHA-256 checksum bs58: 2onZk3T9QqqNTEMwHf6EGBtLUEa4WyebtxDfYzhq5mLW
    - nearblocks account link [product.simple-factory-with-features-verify-rs-ci-a.testnet](https://testnet.nearblocks.io/address/product.simple-factory-with-features-verify-rs-ci-a.testnet?tab=contract)
      - expected source-scan verification error ❌ **Error occurred during command...**
        (due to missing `[package.metadata.near.reproducible_build]` in manifest for `simple-factory-product` and yet incorrect source-scan verification code)
