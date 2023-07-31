declare module "surreal-neon-test" {

    export class Database {
        private constructor();
    }

    export function new_db(address: string): Promise<Database>;
    export function query(handle: Database, query: string): Promise<string>;
    export function use_ns(handle: Database, ns: string): Promise<void>;
    export function use_db(handle: Database, db: string): Promise<void>;
}
