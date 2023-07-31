const express = require('express');
const bodyParser = require('body-parser');
const { new_db, query, use_ns, use_db } = require('.');

const app = express();
app.use(express.static('public'));

// Use the body-parser middleware to parse JSON bodies
app.use(bodyParser.json());

let db;

async function initDB() {
    db = await new_db(process.cwd() + "/database");
    await use_ns(db, "test");
    await use_db(db, "test");
}

initDB().catch(err => console.error(err));

app.get('/', (req, res) => {
    res.send('this is jd`s test surrealdb rig');
});

app.post('/sql', async (req, res) => {
    try {
        let reply = await query(db, req.body.sql);
        res.json(reply);
    } catch (err) {
        console.error(err);
        res.status(500).json({error: 'An error occurred'});
    }
});

app.get('/data', async (req, res) => {
    try {
        let reply = await query(db, "CREATE |test1:10| SET val = rand(), val2 = rand() * 10, val3 = rand::guid(5)");   // "UPDATE |test:100|"
        res.json(reply);
    } catch (err) {
        console.error(err);
        res.status(500).json({error: 'An error occurred'});
    }
});

app.get('/update', async (req, res) => {
    try {
        let reply = await query(db, "UPDATE |test1:10| SET val = rand(), val2 = rand() * 10, val3 = rand::guid(5)");   // "UPDATE |test:100|"
        res.json(reply);
    } catch (err) {
        console.error(err);
        res.status(500).json({error: 'An error occurred'});
    }
});

app.get('/select', async (req, res) => {
    try {
        let reply = await query(db, "SELECT * FROM test");
        res.json(reply);
    } catch (err) {
        console.error(err);
        res.status(500).json({error: 'An error occurred'});
    }
});

app.listen(3002, '0.0.0.0', () => console.log('Server started on port 3002'));
