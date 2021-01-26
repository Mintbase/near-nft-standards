# NEP 4

https://github.com/near/NEPs/pull/4

```js
  type TokenId = u64;
  export function grant_access(escrow_account_id: string): void;
  export function revoke_access(escrow_account_id: string): void;
  export function transfer_from(, owner_id: string, new_owner_id: string, tokenId: TokenId): void;
  export function transfer(new_owner_id: string, token_id: TokenId): void;
  export function check_access(token_id: TokenId): boolean;
  export function get_token_owner(token_id: TokenId): string;
```

```rust
type TokenId = u64;
type AccountId = String;
pub trait NEP4 {
    fn grant_access(&mut self, escrow_account_id: AccountId);
    fn revoke_access(&mut self, escrow_account_id: AccountId);
    fn transfer_from(&mut self, owner_id: AccountId, new_owner_id: AccountId, tokenId: TokenId);
    fn transfer(&mut self, new_owner_id: AccountId, token_id: TokenId);
    fn check_access(&self, token_id: TokenId) -> bool;
    fn get_token_owner(&self, token_id: TokenId) -> AccountId;
}
```
