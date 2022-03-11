##This command line programme creates a SHA256 hash from a file.

This can be used together with the BIP39 command line tool when a file is used to create a SEED Phrase.

My thinking is here that if you write down a seed phrase, it will be obvious that this is can be used to restore a wallet, using some file as an input to create a seed phrase is not so obvious.

Ofcourse you need to make sure you always have access to the input file and that the input file is not changed, hence the creation of a SHA256 has from a file so that you note down the file and the SHA256 hash.

When you need to restore you take the file, take the hash and compare, if equal the seed phrase generated should be the same (This needs to be tested).

PLEASE DO YOUR OWN RESEARCH AND REVIEW THIS PROGRAM BEFORE YOU COMMIT TO ANYTHING!
