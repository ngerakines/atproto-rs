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
