# Nearfolio extensions

```rust
pub struct NonFungibleToken {
    pub owner_id: AccountId,
    pub current_supply: u64,
    pub total_editions: u64,
    pub total_collections: u64,
    pub minters: UnorderedSet<AccountId>,
    pub metadata: UnorderedMap<TokenId, Metadata>,
    pub tokens: UnorderedMap<TokenId, Token>,
    pub collections: UnorderedMap<CollectionId, Collection>,
    pub editions: UnorderedMap<u64, Edition>,
    pub marketplace: UnorderedMap<EditionNumber, TokenPrice>,
    pub account_to_editions: UnorderedMap<AccountId, u64>,
    pub account_gives_access: UnorderedMap<AccountId, UnorderedSet<AccountId>>,
    pub offers: UnorderedMap<String, Vector<Bid>>,
    // Vec<u8> is sha256 of account, makes it safer and is how fungible token also works
    pub events: Vector<Event>,
    pub mint_storage_fee: Balance,
    pub create_collection_fee: Balance,
}

pub trait Nearfolio{
  fn add_minter(&mut self, minter: AccountId);
  fn remove_minter(&mut self, minter: AccountId);
  #[payable]
  fn mint_token(&mut self, mut metadata: Metadata);
  fn burn_token(&mut self, token_id: TokenId, edition_id: EditionNumber);
  #[payable]
  fn create_collection(&mut self, mut collection: Collection);
  fn set_price(&mut self, token_id: TokenId, edition_id: EditionNumber, price_as_yoctonear: String);
  fn cancel_sale(&mut self, token_id: TokenId, edition_id: EditionNumber);
  #[payable]
  fn offer(&mut self, token_id: TokenId, edition_id: EditionNumber);
  fn accept_offer(&mut self, token_id: TokenId, edition_id: EditionNumber, idx: u64);
  fn cancel_offer(&mut self, token_id: TokenId, edition_id: EditionNumber, idx: u64);
  fn gen_token_x_edition(&self, token_id: TokenId, edition_id: EditionNumber) -> String;
  fn get_offers(&self, token_id: TokenId, edition_id: EditionNumber) -> Vec<Bid>;
  fn get_token(&self, token_id: TokenId) -> Token;
  fn get_edition(&self, token_id: TokenId, edition_id: EditionNumber) -> Edition;
  fn get_collection(&self, collection_id: CollectionId) -> Collection ;
  fn balance_of(&self, account: AccountId) -> u64;
  fn get_metadata(&self, token_id: TokenId) -> Metadata;
  fn event(&self, index: usize) -> Event;
  fn events(&self) -> u64;
  fn is_escrow(&self, account_id: AccountId, escrow: AccountId) -> bool;
  fn get_escrows(&self, account_id: AccountId) -> Vec<AccountId;
  fn owner(&self) -> AccountId;
  fn is_minter(&self, account: AccountId) ->  bool;
}
```
