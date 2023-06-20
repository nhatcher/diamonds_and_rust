// node --experimental-wasm-modules test1.mjs
import { iterFact } from './test1.wasm';
console.log(iterFact(BigInt(5)));

