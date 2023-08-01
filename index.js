const surreal = require("./lib/index.node");

class SurrealDB {
    constructor(address) {
        this.db = surreal.new_db(address);
    }

    async ready() {
        await this.db;
    }

    async use(args) {
        if (args.ns !== undefined) {
            surreal.use_ns(await this.db, args.ns);
        }
        if (args.db !== undefined) {
            surreal.use_db(await this.db, args.db);
        }
    }

    async query(query) {
        return await surreal.query(await this.db, query);
    }
}

module.exports = SurrealDB;
