#!/bin/bash 
#
# build_keypair.sh
#
# Create a ECDSA keypair for use with crypto currencies
# The key will be derived from whatever seed phrase is entered by the user
#
# Copyright (c) 2019 B Tasker
#
# https://snippets.bentasker.co.uk/page-1907120828-Create-ECDSA-Keypair-from-Seed-Phrase-BASH.html

read -p "Enter a seed sentence: " seedphrase

# Derive a private key
privkey=$(echo "$seedphrase" | openssl sha256 | cut -d\  -f2)

# Get a proper copy of the private key
privkeyfull=$(openssl ec -inform DER -in <(cat <(echo -n "302e0201010420") <(echo -n "$privkey") <(echo -n "a00706052b8104000a") | xxd -r -p) 2>/dev/null)

# Now start creating the pub key 
longpub=$(openssl ec -inform DER -text -noout -in <(cat <(echo -n "302e0201010420") <(echo -n "$privkey") <(echo -n "a00706052b8104000a") | xxd -r -p) 2>/dev/null  | tail -6 | head -5 | sed 's/[ :]//g' | tr -d '\n' && echo)

# Create the compressed version
compub=$(echo -n "$longpub" | cut -c1-66 | sed 's/^04/02/')

# Now RipeMD it:
hash256=$(echo "$compub" | xxd -r -p | openssl sha256 | cut -d\  -f2)
ripemd=$(echo "$hash256" | xxd -r -p | openssl ripemd160 | cut -d\  -f2)

# Now RipeMD the uncompressed :
hash256=$(echo "$longpub" | xxd -r -p | openssl sha256 | cut -d\  -f2)
ripemdunc=$(echo "$hash256" | xxd -r -p | openssl ripemd160 | cut -d\  -f2)

# And a version we can pass into OpenSSL if we need to
pubkeyfull=$(openssl ec -inform DER -in <(cat <(echo -n "302e0201010420") <(echo -n "$privkey") <(echo -n "a00706052b8104000a") | xxd -r -p) -pubout 2>/dev/null)

cat << EOM
Seed Phrase: 
$seedphrase

Keys:

Private: $privkey
Long public: $longpub
Compressed Public: $compub
RipeMD (Compressed) Public: $ripemd
RipeMD (Uncompressed) Public: $ripemdunc

PEMs:

$privkeyfull
$pubkeyfull

Keep these safe
EOM
