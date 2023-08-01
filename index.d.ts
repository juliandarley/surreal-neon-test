declare module "surreal-neon-test" {
    export default class SurrealDB {
        constructor(address: string);
        ready(): Promise<void>;
        use(args: { ns?: string, db?: string }): Promise<void>;
        query(query: string): Promise<string>;
    }
}
