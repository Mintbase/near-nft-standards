// NOTE: This API is out of date, as of Feb 5, 2021.
// Last updated on Jan 30, 2021.

pub struct MintbaseMint {
    marketplace_address: AccountId,
    minters: LookupSet<AccountId>,
    mint_description: MintDescription,
    tokens: LookupMap<TokenId, TokenDescription>,
    permissioners: LookupMap<AccountId, LookupSet<AccountId>>,
    tokens_minted: u64,
    tokens_burned: u64,
}

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

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct Royalty {
  pub split_between: HashMap<AccountId, Fraction>,
  pub percentage: Fraction,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct Fraction {
  pub numerator: u32,
  pub denominator: u32,
}

pub struct MintDescription {
    pub mint_id: MintId,
    pub owner_id: AccountId,
    pub icon_base64: Option<String>,
    pub symbol: Option<String>,
    pub base_uri: String,
}

/// This trait is the Mintbase Mint V1 API, minus the NEP4 Methods.
pub trait Mintbase {
    //////////////////////////////
    // Mint Owner Only Methods //
    //////////////////////////////
    /// Set the marketplace this mint lists tokens to.
    fn set_marketplace(&mut self, market_address: AccountId);

    /// Modify privileges of `account_id`. Minters are able to mint tokens on this
    /// `MintbaseMint`.
    fn grant_minter(&mut self, account_id: AccountId);

    /// Modify privileges of `account_id`. Minters are able to mint tokens on this
    /// `MintbaseMint`.
    fn revoke_minter(&mut self, account_id: AccountId);

    /// Transfer ownership of mint to new owner. Setting `keep_old_minters`
    /// allows all existing minters (including the prior owner) to continue to
    /// mint tokens.
    fn transfer_mint_ownership(&mut self, new_owner: AccountId, keep_old_minters: bool);

    /// `icon_base64` is best understood as the mint logo/icon. Only the mint
    /// owner may update it.
    fn set_icon_base64(&mut self, base64: Option<String>);

    /// The `base_uri` for the mint is the identifier used to look up the mint
    /// on Arweave. Changing the `base_uri` requires the mint owner to be
    /// responsible for making sure their mint location is maintained by their
    /// preferred storage provider.
    fn set_base_uri(&mut self, base_uri: String);

    /// Grant permission for `account_id` to transact `token_id` on user's
    /// behalf. Caller must own `token_id`. Initialize the set of
    /// permissioned accounts if this is the first to be added for this token.
    fn token_grant_access(&mut self, account_id: AccountId, token_id: TokenId);

    /// Revoke access for `account_id` to transact `token_id` on user's
    /// behalf. Caller must own `token_id`.
    fn token_revoke_access(&mut self, account_id: AccountId, token_id: TokenId);

    /// Call `transfer` for each `token_id` in `token_ids` to `account_id`. Note
    /// that each transfer will be individually logged.
    fn batch_transfer(&mut self, to: AccountId, token_ids: Vec<TokenId>);

    /// Call `transfer` for each `token_id` in `token_ids` to the corresponding
    /// `account_id`
    fn batch_heterogeneous_transfer(&mut self, token_ids: Vec<(TokenId, AccountId)>);

    //////////////////////////
    // Minters Only Methods //
    //////////////////////////
    /// The core mint function. Mint token will mint `num_to_mint` copies of a
    /// token with `meta_id`, assigning `owner_id` as the owner of the token. Note
    /// that `meta_id`'s are not required to be unique. Users from the minter set
    /// (including the owner) may call this function.
    ///
    /// For serialization simplicity, the Royalty field arguments given as f32
    /// (not Fraction), and converted to the safe-to-multiply Fraction type.
    fn mint_token(
        &mut self,
        owner_id: AccountId,
        meta_id: MetaId,
        num_to_mint: u64,
        roy: Option<HashMap<AccountId, f32>>,
        roy_f: f32,
    );

    /////////////////////////////////////////
    // Cross Contract Calls to Marketplace //
    /////////////////////////////////////////
    /// List a token to the Marketplace mintd on this mint contract. The
    /// Marketplace contract is expected to have a method, receive_list_token.
    fn list_token(
        &mut self,
        token_id: TokenId,
        autotransfer: bool,
        asking_price: Balance,
        split_owners: Option<HashMap<AccountId, f32>>,
    );

  /// List several tokens to the Marketplace mintd on this mint contract. The
	/// Marketplace contract is expected to have a method, receive_batch_list_token.
    fn list_token(
      &mut self,
      permissions_intermediary_address: Option<AccountId>,
      market_address: AccountId,
      token_ids: Vec<TokenId>,
      autotransfer: bool,
      asking_price: Balance,
      split_owners: Option<HashMap<AccountId, f32>>,
    );

    ///////////////////////////////////////////
    // (User|Token) Gives Permission Methods //
    ///////////////////////////////////////////

    // transfer_from in NEP4

    /// Call `transfer_from` for each `token_id` in `token_ids` to `account_id`.
    /// Each transfer will be individually logged.
    fn batch_transfer_from(&mut self, token_ids: Vec<TokenId>, to: AccountId);

    fn burn_token(&mut self, token_id: TokenId);

    /// Call `burn_token` for each `token_id` in `token_ids`. Each burn will be
    /// individually logged.
    fn batch_burn(&mut self, token_ids: Vec<TokenId>);

    ///////////////////////////////
    // Mint Interfacing Getters //
    ///////////////////////////////
    /// If caller is minter, return true.
    fn am_minter(&self) -> bool;

    /// Check if `account_id` is a minter.
    fn check_is_minter(&self, account_id: AccountId) -> bool;

    /// Get the number of tokens minted to date.
    fn get_num_minted(&self) -> u64;

    /// Get the number of tokens burned to date.
    fn get_num_burned(&self) -> u64;

    /// If `account_id` has permission to transact on behalf of `on_behalf`,
    /// return true.
    fn check_self_or_permissioner(&self, account_id: AccountId, on_behalf: AccountId) -> bool;

    /// If caller has permission to transact on behalf of `on_behalf`, return
    /// true.
    fn am_self_or_permissioner(&self, on_behalf: AccountId) -> bool;

    /// Get the `mint_id` of this mint.
    fn get_mint_id(&self) -> &MintId;

    /// Get the `owner_id` of this mint.
    fn get_mint_owner(&self) -> &AccountId;

    /// Get the `icon_base64` of this mint.
    fn get_mint_icon(&self) -> &Option<String>;

    /// Get the `base_uri` of this mint.
    fn get_mint_base_uri(&self) -> &String;

    /// Get the `symbol` of this mint.
    fn get_mint_symbol(&self) -> &Option<String>;
}

///////////////////////////////
// Token Interfacing Getters //
///////////////////////////////
#[near_bindgen]
impl MintbaseMint {
    /// Get the `minter` for token with `token_id`.
    fn get_token_minter(&self, token_id: TokenId) -> AccountId;

    /// Get the `meta_id` for token with `token_id`.
    fn get_token_meta_id(&self, token_id: TokenId) -> MetaId;

    /// Token URI is generated to index the token on whatever distributed storage
    /// platform the mint uses (Arweave by Mintbase default, though mint owners
    /// may opt into their own).
    fn get_token_uri(&self, token_id: TokenId) -> String;

    /// Get the number of copies originally in existance of token with `token_id`.
    /// `get_num_copies` is blind to whether the number of copies that have been
    /// burned.
    fn get_num_copies(&self, token_id: TokenId) -> u64;

    /// If `account_id` has permission to transact for `token_id`, return true.
    fn check_owner_or_token_permissioner(&self, token_id: TokenId, account_id: AccountId) -> bool;

    /// If caller has permission to transact for `token_id`, return true.
    fn am_token_permissioner(&self, token_id: TokenId) -> bool;

    /// Get Royalty for given token
    fn get_royalty(&self, token_id: TokenId) -> Option<Royalty>;

    /// Get the `token_unique_id` for `token_id`. The `token_unique_id` is the
    /// immutable combination of the token's `token_id` (unique within this
    /// mint), and the Mint address. The `token_id` distinguishes tokens from
    /// one another within the mint, but not between mints. The UniqueId is
    /// unique across mints.
    fn get_token_unique_id(&self, token_id: TokenId) -> UniqueId;
}
