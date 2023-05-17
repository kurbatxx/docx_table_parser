-- @block create node table
DROP TABLE IF EXISTS node;
CREATE TABLE node(
    node_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    parrent_id integer,
    node_name text
);
INSERT INTO node (parrent_id, node_name)
VALUES (0, 'Казахстан'),
    (0, 'Россия');
--
--
CREATE OR REPLACE FUNCTION set_kz () RETURNS void language plpgsql as $$
DECLARE id integer;
BEGIN
SELECT node_id into id
FROM node
WHERE node_name = 'Казахстан';
--
INSERT INTO node (parrent_id, node_name)
VALUES (id, 'Актюбинская область'),
    (id, 'Акмолинская область'),
    (id, 'Алматинская область'),
    (id, 'Атырауская область'),
    (id, 'Восточно-Казахстанская область'),
    (id, 'Жамбылская область'),
    (id, 'Жетысуская область'),
    (id, 'Западно-Казахстанская область'),
    (id, 'Карагандинская область'),
    (id, 'Костанайская область'),
    (id, 'Кызылординская область'),
    (id, 'Мангистауская область'),
    (id, 'Павлодарская область'),
    (id, 'Северо-Казахстанская область'),
    (id, 'Туркестанская область'),
    (id, 'Улытауская область');
END;
$$;
SELECT set_kz ();
-- @block select nodes with current parrent
SELECT node_id,
    parrent_id,
    node_name
FROM node
WHERE parrent_id = 1