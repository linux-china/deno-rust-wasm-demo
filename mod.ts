import init, {
    source,
    add as wasm_add,
    hello as wasm_hello,
    fetch_ip
} from "./wasm.js";

await init(source);

export function add(a: number, b: number): number {
    return wasm_add(a, b);
}

export function hello(name: string): string {
    return wasm_hello(name);
}

export function fetchIp(): {origin: string} {
    return fetch_ip();
}
