const runeStoneDecoder = require('./pkg/decoder.js');

const safeDecipher = (hex) => {
    try {
        return runeStoneDecoder.get_runestone_from_transaction_hex(hex);
    } catch (e) {

        return null;
    }

}

module.exports = {
    decipher: safeDecipher
}