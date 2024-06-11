// [1,2].forEach((v) => {
//     if (v === 1) {
//         return;
//     }
//     console.info(v);
// })

const o = {};

let c = o;
for (let i = 0; i < 5; i++) {
    if (typeof c !== 'object') {
        continue;
    }
    if (c[i] === undefined)  {
        c[i] = {};
    }
    
    c = c[i];
}
console.info(o);