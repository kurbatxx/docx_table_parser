DROP TABLE IF EXISTS node;
CREATE TABLE node(
    node_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    parrent_id integer,
    node name text UNIQUE
);