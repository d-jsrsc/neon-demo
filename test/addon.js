const addon = require('..');

console.log(addon.hello());

const buf = Buffer.from('12');
console.log('--', buf)
console.log(addon.jsBufferTo(buf));
console.log('++', buf)
console.log((function() {
    let buf = addon.bufferFromRs()
    return buf.readBigUInt64BE(0);
})());