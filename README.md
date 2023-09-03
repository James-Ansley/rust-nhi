# NHI

Checks strings against the New Zealand Ministry of Health NHI Validation Routine.
Supports the old and new NHI number formats specified in
[HISO 10046:2023](https://www.tewhatuora.govt.nz/publications/hiso-100462023-consumer-health-identity-standard/).

## Install

```
cargo add nhi
```

## Docs

- <https://docs.rs/nhi/latest/nhi/>

## Example

NHI values can be validated with the `is_nhi` function, or parsed to `NHI` structs:

```rust
use nhi::{is_nhi, NHI};

fn main() {
    let nhi_str = "zac5361";

    assert_eq!(is_nhi(nhi_str), true);
    
    let nhi: NHI = nhi_str.parse().unwrap();
    assert_eq!(nhi.as_str(), nhi_str.to_uppercase());
}
```

More examples are available in [the docs](https://docs.rs/nhi/latest/nhi/).


## See Also

- <https://www.tewhatuora.govt.nz/publications/hiso-100462023-consumer-health-identity-standard/>
- <https://www.tewhatuora.govt.nz/our-health-system/digital-health/health-identity/national-health-index/information-for-health-it-vendors-and-developers>
