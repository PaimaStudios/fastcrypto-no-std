Both examples here are using the pietrzak VDF using RSA groups.

Run `rust-prove-vdf` normally to generate the prove of the VDF.

Then run `nexus-verify-vdf` in a zk-setting using `cargo nexus prove` to generate a zk proof of the verification.