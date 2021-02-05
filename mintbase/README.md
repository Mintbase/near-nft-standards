# What Mintbase considers important out of this spec:

 1. An `Option<Royalty>` field on each Token means Markets may query Royalty
 information intended by the original token creator.

 2. the `list_token` and `batch_list_token` methods ought to be synchronized
 across token-producer contracts and market contracts, so that token factories
 and markets may interoperably list one another's tokens.

3. Token level permissions to grant and revoke permission to transfer a  token, as opposed te the NEP4 standard which only allows account-level permissions.

# Core Points: Store
Mintbase's full mint API (currently unstable) can be found in `mb_mint.rs`.

This section contains a list of API details relevant to the Feb 5, 2021 meeting of the Near NFT Standards Committee.

## Standardize Token Metadata
```rust
pub struct TokenDescription {
  /// The account that minted this token.
  pub(crate) minter: AccountId,
  /// The current owner of this token.
  pub(crate) owner_id: AccountId,
  /// The numerical identifier of this token.
  pub(crate) token_id: TokenId,
  /// An Arweave string to get index of attached media content.
  pub(crate) meta_id: String,
  /// The number of copies of this token in existance. Does not tracked burned.
  pub(crate) copies: u64,
  /// The accounts that have permission to call transfer on this token.
  pub(crate) permissioners: LookupSet<AccountId>, // Has to be a LookupSet: UnorderedSet has dynamic size.
  /// Feature for Markets listing this token. This field represents the
  /// royalty asked for by the token minter. Markets may choos to respect or
  /// ignore this field. Royalties are set once when the token is minted, and
  /// cannot be changed.
  pub(crate) royalty: Option<Royalty>,
}
```


## Standardize NFT Producers
```rust
pub struct MintbaseMint {
  /// Accounts that are allowed to mint tokens on this Mint.
  minters: LookupSet<AccountId>,
  /// Metadata of this Mint.
  mint_description: MintDescription,
  /// Tokens this Mint has minted.
  tokens: LookupMap<TokenId, TokenDescription>,
  /// A mapping for each User to the set of Addresses that may transact with
  /// that user's tokens.
  permissioners: LookupMap<AccountId, LookupSet<AccountId>>,
  /// The number of tokens this Mint has minted.
  tokens_minted: u64,
  /// The number of tokens this Mint has burned.
  tokens_burned: u64,
}
```

## NEP4 API
```rust
pub trait NEP4 {
  fn grant_access(&mut self, escrow_account_id: AccountId);
  fn revoke_access(&mut self, escrow_account_id: AccountId);
  /// `transfer_from` has an unnecessary field: `owner_id`
  fn transfer_from(&mut self, owner_id: AccountId, new_owner_id: AccountId, tokenId: TokenId);
  /// Transfer is redundant, given the existance of `transfer_from`.
  fn transfer(&mut self, new_owner_id: AccountId, token_id: TokenId);
  fn check_access(&self, token_id: TokenId) -> bool;
  fn get_token_owner(&self, token_id: TokenId) -> AccountId;
}
```

## Mintbase's Non-Market related Proposed Extensions to the NEP4 API
It does not seem necessary to standardize the `mint_token` interface. However, certain methods would benefit from standardization. In particular, batch methods to improve gas efficiency, and royalty getters.
```rust
/// The globally unique identifier for a token. Mintbase's global standard for
/// formatting this unique identifier based on the TokenId on a Mint contract,
/// concatenated to the Mint address
pub type UniqueId = String;
pub trait MB{
  /// Call `transfer_from` for each `token_id` in `token_ids` to `account_id`. Note
  /// that each transfer will be individually logged.
  pub fn batch_transfer_from(&mut self, token_ids: Vec<(AccountId, TokenId)>);
  /// get the Royalty field of this token
  pub fn get_token_royalty(&self, token_id: TokenId) -> Option<Royalty>;
  /// get the number of editions/copies of this token ref: ERC1155
  pub fn get_copies(&self, token_id: TokenId) -> u64;
  /// get the globally unique identifier of a token
  pub fn get_token_unique_id(&self, token_id: TokenId) -> UniqueId {
    format!("{}-{}", token_id, env::current_account_id())
  }
  /// Token URI is generated to index the token on whatever distributed storage
  /// platform the Mint uses (Arweave by Mintbase default).
  pub fn get_token_uri(&self, token_id: TokenId) -> String {
    let base = &self.get_mint_description().base_uri;
    let meta_id = self.get_token(token_id).meta_id;
    format!("{}/{}", base, meta_id)
  }
```

## Mintbase's Market-related Proposed Extensions to the NEP4 API

```rust
pub trait MBM{
  /// List a batch of tokens to a marketplace. The batch may be of size 1. All
  /// tokens listed must be owned by the caller of this function.
  ///
  /// These fields on the tokens listed in the batch MUST be identical for each
  /// token:
  /// - Royalty
  /// - OwnerId
  ///
  /// This method MAY include an `Option` to include a permissions intermediary
  /// contract. A permissions intermidary contract holds permissions for users,
  /// so that NFTs themselves do not need to store uneccessary permissions data.
  /// Further, a Permissions contract facilitates market upgrade migrations and
  /// multiple marketplace interactions by abstracting permissions behavior.
  ///
  /// This method calls a reciprocal method, `receive_list_token` on the
  /// `permissions_intermediary` contract, if there is one, or on the `market`
  /// contract if there is not.
  pub fn list_token(
    &mut self,
    permissions_intermediary_address: Option<AccountId>,
    market_address: AccountId,
    token_ids: Vec<TokenId>,
    autotransfer: bool,
    asking_price: U128,
    split_owners: Option<HashMap<AccountId, f32>>,
  );

  /// Delist a batch of tokens from a marketplace. The batch may be of size 1. All
  /// tokens listed must be owned by the caller of this function.
  ///
  /// These fields on the tokens listed in the batch MUST be identical for each
  /// token:
  /// - OwnerId
  ///
  /// This method MAY include an `Option` to include a permissions intermediary
  /// contract. A permissions intermidary contract holds permissions for users,
  /// so that NFTs themselves do not need to store uneccessary permissions data.
  /// Further, a Permissions contract facilitates market upgrade migrations and
  /// multiple marketplace interactions by abstracting permissions behavior.
  ///
  /// This method calls a reciprocal method, `receive_list_token` on the
  /// `permissions_intermediary` contract, if there is one, or on the `market`
  /// contract if there is not.
  pub fn delist_token(
    &mut self,
    permissions_intermediary_address: Option<AccountId>,
    market_address: AccountId,
    token_ids: Vec<TokenId>,
    autotransfer: bool,
    asking_price: U128,
    split_owners: Option<HashMap<AccountId, f32>>,
  );
}
```

# Permissions Intermediary Contract
```rust
pub struct MintbasePermissions {
  /// The address that may change the trusted market settings
  owner_id: AccountId,
  /// Set of markets this escrow contract trusts
  trusted_markets: LookupSet<AccountId>,
  /// Tokens this contract has permission for (format: TOKEN_ID-MINT_ID)
  token_permissions: LookupSet<UniqueId>,
}

  fn receive_list_token(
    &mut self,
    owner_id: AccountId,
    token_ids: Vec<TokenId>,
    autotransfer: bool,
    asking_price: U128,
    royalties: Option<Royalty>,
    split_owners: Option<HashMap<AccountId, f32>>,
  );
  fn receive_delist_token(
    &mut self,
    owner_id: AccountId,
    token_ids: Vec<TokenId>,
    autotransfer: bool,
    asking_price: U128,
    royalties: Option<Royalty>,
    split_owners: Option<HashMap<AccountId, f32>>,
  );

```


# Secondary Points
## Standardize Token Producer Contract Metadata
```rust
pub struct MintDescription {
    pub mint_id: MintId,
    pub owner_id: AccountId,
    pub icon_base64: Option<String>,
    pub symbol: Option<String>,
    pub base_uri: String,
}
```
