# ERC 721 Reference
This document serves as reference to the original ERC721 standard, as it is
ported to the NEAR ecosystem. It is given as originally provided, then
translated into Rust. Comments are removed to be concise.

Ref: https://eips.ethereum.org/EIPS/eip-721.

## ERC721
```js
interface ERC721 /* is ERC165 */ {
    event Transfer(address indexed _from, address indexed _to, uint256 indexed _tokenId);
    event Approval(address indexed _owner, address indexed _approved, uint256 indexed _tokenId);
    event ApprovalForAll(address indexed _owner, address indexed _operator, bool _approved);
    function balanceOf(address _owner) external view returns (uint256);
    function ownerOf(uint256 _tokenId) external view returns (address);
    function safeTransferFrom(address _from, address _to, uint256 _tokenId, bytes data) external payable;
    function safeTransferFrom(address _from, address _to, uint256 _tokenId) external payable;
    function transferFrom(address _from, address _to, uint256 _tokenId) external payable;
    function approve(address _approved, uint256 _tokenId) external payable;
    function setApprovalForAll(address _operator, bool _approved) external;
    function getApproved(uint256 _tokenId) external view returns (address);
    function isApprovedForAll(address _owner, address _operator) external view returns (bool);
}

interface ERC165 {
    function supportsInterface(bytes4 interfaceID) external view returns (bool);
}

interface ERC721TokenReceiver {
    function onERC721Received(address _operator, address _from, uint256 _tokenId, bytes _data) external returns(bytes4);
}

interface ERC721Metadata /* is ERC721 */ {
    function name() external view returns (string _name);
    function symbol() external view returns (string _symbol);
    function tokenURI(uint256 _tokenId) external view returns (string);
}
```

## METADATA JSON SCHEMA
This may be kept as identical or modified as seen fit.
```json
{
    "title": "Asset Metadata",
    "type": "object",
    "properties": {
        "name": {
            "type": "string",
            "description": "Identifies the asset to which this NFT represents"
        },
        "description": {
            "type": "string",
            "description": "Describes the asset to which this NFT represents"
        },
        "image": {
            "type": "string",
            "description": "A URI pointing to a resource with mime type image/* representing the asset to which this NFT represents. Consider making any images at a width between 320 and 1080 pixels and aspect ratio between 1.91:1 and 4:5 inclusive."
        }
    }
}
```

## Rust Equivalent:
Note that Rust has no native u256 type. Possible replacements for u256: Either
generate a u256, or replace it with a smaller, u128 or u64.

In place of `event`s, logger functions are given.

```rust
pub type TokenId = u256; // *
pub type AccountId = String;
pub type Balance = u128;
pub trait ERC721  {
  fn log_transfer(from: AccountId, to: AccountId, tokenId: TokenId);
  fn log_approval(owner: AccountId, approved: AccountId, tokenId: TokenId);
  fn log_approval_for_all(owner: AccountId, operator: AccountId, approved: bool);

  fn balanceOf(&self, owner: AccountId) -> Balance; // *
  fn ownerOf(&self, tokenId: TokenId) -> AccountId;
  #[payable]
  fn safe_transfer_from(&mut self, from: AccountId, to: AccountId, tokenId: TokenId); // *
  #[payable]
  fn transfer_from(&mut self, from: AccountId, to: AccountId, tokenId: tokenId);
  #[payable]
  fn approve(&mut self, approved: AccountId, tokenId: TokenId);
  fn set_approval_for_all(&mut self, operator: AccountId, approved: bool);
  fn get_approved(&self, tokenId: TokenId) -> AccountId;
  fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool;
}

pub trait ERC165 {
    fn supports_interface(interfaceID: String) -> bool;
}

pub trait ERC721TokenReceiver {
    fn on_ERC721_received(&self, operator: AccountId, from: AccountId, tokenId: TokenId, data: String) -> String;
}

pub trait ERC721Metadata {
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn tokenURI(&self, tokenId: TokenId) -> String;
}
pub trait ERC721Enumerable {
    fn total_supply(&self) -> TokenId;
    fn token_by_index(&self, index: TokenId) -> TokenId;
	fn token_of_owner_by_index(&self, owner: AccountId, index: TokenId) -> TokenId;
}
```
