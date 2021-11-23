import init, {
    source,
    add as wasm_add,
    hello as wasm_hello,
    hello_boa,
    fetch_ip
} from "./wasm-bundle.js";

await init(source);

export function add(a: number, b: number): number {
    return wasm_add(a, b);
}

export function hello(name: string): string {
    return wasm_hello(name);
}

export function helloBoa(name: string): string {
    return hello_boa(name);
}

export function fetchIp(): {origin: string} {
    return fetch_ip();
}
