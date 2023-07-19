const { new_db, query, use_ns, use_db } = require('.');


async function run() {
    const db = await new_db(process.cwd() + "/database");

    await use_ns(db, "test");
    await use_db(db, "test");

    let reply = await query(db, "UPDATE |test:100|");
    console.log(reply);
}
run();