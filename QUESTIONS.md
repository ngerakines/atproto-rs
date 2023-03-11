# Questions

This is me just trying to figure stuff out.

# Handle Exists

A handle may exist, but it must support some set of resolution methods.

For example, `nick.cauda.cloud` would exist at `https://nick.cauda.cloud/`. The DID would be resolved through an XRPC request (not `.well-known/did.json`) and would reference the PDS serving the handle.

# PDS Exists

In this scenario, a PDS exists at `https://cauda.cloud`.

The PDS will have one more more keys that it uses. At least one of these keys is provided for public key verification of signatures. Some of these keys may be "offline" keys for recovery purposes.

Implications:

* The server should support multiple signing keys
* The server should support requests to `GET /.well-known/jwks.json`

# User creation

In this scenario, a PDS exists at `https://cauda.cloud`.

A user wants to create an account, `nick.cauda.cloud`. First they generate a recovery key:

```python
from multibase import encode, decode
from ecdsa import SigningKey, VerifyingKey, SECP256k1

if __name__ == "__main__":
    sk = SigningKey.generate(curve=SECP256k1)
    print("private={key}".format(key=encode("base58btc", sk.to_string()).decode("utf-8")))
    vk = sk.verifying_key
    signature = sk.sign(b"message")
    encoded_verifying_key = encode("base58btc", vk.to_string())
    print("public={key}".format(key=(encoded_verifying_key.decode("utf-8"))))
    new_vk = VerifyingKey.from_string(decode(encoded_verifying_key), curve=SECP256k1)
    assert new_vk.verify(signature, b"message")
```

The generate private and public key is stored securely by the user and the public key sent as a recovery_key parameter to the create account xrpc method:

```
private=z7F4fw9jCXtW3qxi6c3ew7y7LDRppvumSQc7Tp3bvzCyV
public=z2ZaygJJjGTMRYZDyHqLZwv7qneUDwu99X11JQ7DK8Z4ma28Ux9zFcW3qpMengmmxicBfMpqoDCu33EFiTPpxVb9y
```

The PDS is the first home for the user and the xprc method `com.atproto.account.create` is invoked as:

```json
{
  "$type": "com.atproto.account.create",
  "handle": "nick",
  "email": "nick@theservice.amazing",
  "password": "secret_password",
  "recovery_key": "z2ZaygJJjGTMRYZDyHqLZwv7qneUDwu99X11JQ7DK8Z4ma28Ux9zFcW3qpMengmmxicBfMpqoDCu33EFiTPpxVb9y",
}
```

Internally, the PDS creates a create operation:

```json
{

  "type": "create",
  "services": {
    "atproto_pds": {
      "type": "AtprotoPersonalDataServer",
      "endpoint": "https://cauda.cloud"
    }
  },
  "alsoKnownAs": [
    "at://nick.cauda.cloud"
  ],
  "rotationKeys": [
    "did:key:_pds_signing_key_active",
    "did:key:_pds_signing_key_offline",
    "did:key:_user_recovery_key"
  ],
  "verificationMethods": {
    "atproto": "did:key:_pds_signing_key_active"
  }
}
```

That create operation is used to create the DID of the user based on `did:plc:${base32Encode(sha256(createOp)).slice(0,24)}`.

This can be computed with the following:

```python
import hashlib, base64

if __name__ == "__main__":
    sha256 = hashlib.sha256()
    with open("nick.cauda.cloud.create.json", "rb") as f:
        sha256.update(f.read())
    digest = sha256.digest()
    encoded_digest = base64.b32encode(digest)
    print(encoded_digest[:min(len(encoded_digest), 24)].lower())
```

The resulting did for the user is `did:plc:dpthpm5cw3f6vc5qbcmcfjc7`.

The create operation is also signed and made available as a public log of operations.

TBD: Show how the create entry is signed.

The following identifiers are all valid for the user:

* `nick.cauda.cloud`
* `at://nick.cauda.cloud`
* `https://nick.cauda.cloud`
* `at://did:plc:dpthpm5cw3f6vc5qbcmcfjc7`

The user can be resolved by making requests to the PDS:

* `GET https://X/xrpc/com.atproto.handle.resolve?handle=Y`
  * Where X is one of `cauda.cloud` or `nick.cauda.cloud`
  * Where Y is one of
    * `nick`
    * `nick.cauda.cloud`
    * `did:plc:dpthpm5cw3f6vc5qbcmcfjc7`

The following DID would exist for that handle:

```json
{
  "@context": [
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/secp256k1-2019/v1"
  ],
  "id": "did:plc:dpthpm5cw3f6vc5qbcmcfjc7",
  "alsoKnownAs": [
    "at://ngerakines.me"
  ],
  "verificationMethod": [
    {
      "id": "#active-kid-1678556449",
      "type": "EcdsaSecp256k1VerificationKey2019",
      "controller": "did:plc:dpthpm5cw3f6vc5qbcmcfjc7",
      "publicKeyMultibase": "_pds_signing_key_active"
    },
    {
      "id": "#offline-kid-1678556449",
      "type": "EcdsaSecp256k1VerificationKey2019",
      "controller": "did:plc:dpthpm5cw3f6vc5qbcmcfjc7",
      "publicKeyMultibase": "_pds_signing_key_offline"
    },
    {
      "id": "#user-kid-1678556449",
      "type": "EcdsaSecp256k1VerificationKey2019",
      "controller": "did:plc:dpthpm5cw3f6vc5qbcmcfjc7",
      "publicKeyMultibase": "z2ZaygJJjGTMRYZDyHqLZwv7qneUDwu99X11JQ7DK8Z4ma28Ux9zFcW3qpMengmmxicBfMpqoDCu33EFiTPpxVb9y"
    }
  ],
  "rotationKeys": [
    "#active-kid-1678556449",
    "#offline-kid-1678556449",
    "#user-kid-1678556449"
  ],
  "assertionMethod": [
    "#active-kid-1678556449"
  ],
  "capabilityInvocation": [
    "#active-kid-1678556449"
  ],
  "capabilityDelegation": [
    "#active-kid-1678556449"
  ],
  "service": [
    {
      "id": "#atproto_pds",
      "type": "AtprotoPersonalDataServer",
      "serviceEndpoint": "https://bsky.social"
    }
  ]
}
```