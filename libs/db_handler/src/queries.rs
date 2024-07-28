pub const DEVICE_TABLE_QUERY: &str = r#"
CREATE TABLE IF NOT EXISTS Node_Device (
    ID SERIAL PRIMARY KEY,
    mac TEXT NOT NULL,
    name TEXT NOT NULL
);
"#;

pub const PLANT_TABLE_QUERY: &str = r#"
CREATE TABLE IF NOT EXISTS Plants (
    ID SERIAL PRIMARY KEY,
    plant_number INTEGER NOT NULL,
    name TEXT NOT NULL,
    node_device_id INTEGER NOT NULL,
    UNIQUE (plant_number, node_device_id)
);
"#;

pub const LOG_TABLE_QUERY: &str = r#"
CREATE TABLE IF NOT EXISTS Log_Data (
    ID SERIAL PRIMARY KEY,
    plant_id INTEGER NOT NULL,
    device_id INTEGER NOT NULL,
    temp INTEGER NOT NULL,
    humidity INTEGER NOT NULL,
    soil_humidity INTEGER NOT NULL,
    light INTEGER NOT NULL
);
"#;
