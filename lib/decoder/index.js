const runeStoneDecoder = require('./pkg/decoder.js');

module.exports = {
    decode: runeStoneDecoder.get_runestone_from_transaction_hex
}