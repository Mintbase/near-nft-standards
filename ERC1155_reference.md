# ERC1155

https://eips.ethereum.org/EIPS/eip-1155

```js
interface ERC1155 /* is ERC165 */ {
    event TransferSingle(address indexed _operator, address indexed _from, address indexed _to, uint256 _id, uint256 _value);
    event TransferBatch(address indexed _operator, address indexed _from, address indexed _to, uint256[] _ids, uint256[] _values);
    event ApprovalForAll(address indexed _owner, address indexed _operator, bool _approved);
    event URI(string _value, uint256 indexed _id);
    function safeTransferFrom(address _from, address _to, uint256 _id, uint256 _value, bytes calldata _data) external;
    function safeBatchTransferFrom(address _from, address _to, uint256[] calldata _ids, uint256[] calldata _values, bytes calldata _data) external;
    function balanceOf(address _owner, uint256 _id) external view returns (uint256);
    function balanceOfBatch(address[] calldata _owners, uint256[] calldata _ids) external view returns (uint256[] memory);
    function setApprovalForAll(address _operator, bool _approved) external;
    function isApprovedForAll(address _owner, address _operator) external view returns (bool);
}

interface ERC1155TokenReceiver {
    function onERC1155Received(address _operator, address _from, uint256 _id, uint256 _value, bytes calldata _data) external returns(bytes4);
    function onERC1155BatchReceived(address _operator, address _from, uint256[] calldata _ids, uint256[] calldata _values, bytes calldata _data) external returns(bytes4);
}
```

```rust
pub type TokenId = u256; // *
pub type Balance = u128; // *
pub type AccountId = String; // *
pub trait ERC1155 {
    fn log_transfer_single(
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: TokenId,
        value: Balance,
    );
    fn log_transfer_batch(
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<TokenId>,
        values: Vec<Balance>,
    );
    fn log_approval_for_all(owner: AccountId, operator: AccountId, approved: bool);
    fn log_URI(value: String, id: TokenId);

    fn safe_transfer_from(&mut self, from: AccountId, to: AccountId, id: TokenId);
    fn safe_batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids: Vec<TokenId>,
        values: Vec<Balance>,
    );
    fn balance_of(&self, owner: AccountId, id: TokenId) -> Balance; // *
    fn balance_of_batch(&self, owners: Vec<AccountId>, ids: Vec<TokenId>) -> Vec<TokenId>; // *
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool);
    fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool;
}

pub trait ERC1155TokenReceiver {
    fn on_ERC1155_received(
        &self,
        operator: AccountId,
        from: AccountId,
        id: TokenId,
        value: Balance,
    ) -> &[u8];
    fn on_ERC1155_batch_received(
        &self,
        operator: AccountId,
        from: AccountId,
        ids: Vec<TokenId>,
        values: Vec<Balance>,
    ) -> &[u8];
}
```
