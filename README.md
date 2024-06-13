![alt text](https://i.ibb.co/6WLVrYX/apeutils-banner-1.png)

# Apeutils

Ape utils is a compilation of Rune based utilities created mantained by Runeapes Labs.

## Runestone Decoder

Based on Ord 0.18.5 (and updated on every subsequent release), the Runestone decoder is an up to date version of the native Runestone decipher function on Ord, compiled with WASM to javascript. It takes in a raw transaction hex and returns a JSON representation of the transaction's runestone.

Note: If the transaction hex contains an invalid OpCode, the function will return "null".

```js
const apeutils = require("@runeapes/apeutils");
console.log(
  apeutils.runestone.decipher(
    "02000000000101a5e138a64d8a840a7799eb68394fc25c371366f5ed313984d6626b79c494254a0100000000fdffffff020000000000000000096a5d0614b5b6331450faa4000000000000160014c79022e5d704092f05cce060529744363da32b500247304402202ac067004fc3a8bd9b3a7a58373e2059b5aae2f62d6dfcbb4c77ba284319a23502200f977cbcb08c4456ded4282ff84fe93162c63c35633ef46b43839f8199cf123601210301cd61deef4b704902a697dad6ee116ee701ce9b27a11fc920036a2abc0fa52100000000"
  )
);
/*
Returns the following:

{
  Runestone: {
    edicts: [],
    etching: undefined,
    mint: '842549:80',
    pointer: undefined
  }
}
*/
```
