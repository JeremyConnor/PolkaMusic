# Rights Management Portal

The rights management pallet provides functionality for registering and managing master data about music entrepreneurs in the supply chain between various stakeholders.

NOTE: This pallet implements the aforementionned process in a simplified way, thus it is intended for demonstration purposes and is not audited or ready for production use.

![Rights Management Portal](rmp.png)

## Usage

To get music rights on “Smart Streaming Platform (SSP)”, one must send a transaction with a `rightsMgmtPortal.registerMusic` extrinsic with the following arguments:
- `id` as the Song ID, typically this would be a ISRC code (International Standard Recording Code) or ISWC code (International Standard Musical Work Code) or similar.
- `owner` as the Substrate Account representing the music entrepreneur owning the music rights.
- `props` which is a series of properties describing the music information. Typically, there would at least be a textual description.

### Pallets

This pallet depends on on the [FRAME Timestamp pallet](https://docs.rs/crate/pallet-timestamp).

