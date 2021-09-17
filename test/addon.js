const fs = require('fs');

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

const start = Date.now();
addon.imageGen({imgx: 1000, imgy: 1000}, function(err, buf) {
    console.log('-=-=',Date.now() - start)
    console.log({err});
    const r = fs.writeFileSync('./data.jpg', buf, {encoding: 'binary'});
    console.log(r);
});