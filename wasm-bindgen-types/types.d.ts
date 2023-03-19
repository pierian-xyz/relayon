/* TypeScript definitions for p2p_wasm_router */
export class Router {
    constructor();
    static with_key(private_key: string): Router;
    get_public_key(): string;
    connect(signal: string): void;
    send_message(message: string): void;
}
