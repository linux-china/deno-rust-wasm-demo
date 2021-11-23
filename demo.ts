import {add, hello, fetchIp, helloBoa} from "./mod.ts";

addEventListener('demo', () => {
    console.log('Demo event received!');
});

console.log(add(1, 2));
console.log(hello("Jackie"))
console.log(helloBoa("Jackie"))

console.log(await fetchIp());
