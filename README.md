# xorcrypt

A utility for performing a repeated-key xor cipher on files

## Usage

```bash
xorcrypt <inputfile> <outputfile>
```
Decrypting is the exact same command

### !!
XOR ciphers are NOT cryptographically secure, especially not with a 1 letter key <br>
1 letter keys are vulnerable to frequency analysis and are obviously not hard to brute force
