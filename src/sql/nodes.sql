-- @block create node table
DROP TABLE IF EXISTS node;
CREATE TABLE node(
    node_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    parrent_id integer,
    node_name text
);
ALTER TABLE node
ADD CONSTRAINT uniq_node_names_on_level UNIQUE (parrent_id, node_name);
--
--
INSERT INTO node (parrent_id, node_name)
VALUES (0, 'Казахстан'),
    (0, 'Россия');
--
--
CREATE OR REPLACE FUNCTION set_kz () RETURNS void language plpgsql as $$
DECLARE id integer;
id1 integer;
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
--
--
SELECT node_id into id1
FROM node
WHERE node_name = 'Северо-Казахстанская область';
--
--
INSERT INTO node (parrent_id, node_name)
VALUES (id1, 'Петропавловск'),
    (id1, 'Айыртауский район'),
    (id1, 'Акжарский район'),
    (id1, 'Аккайынский район'),
    (id1, 'Есильский район'),
    (id1, 'Жамбылский район'),
    (id1, 'район Магжана Жумабаева'),
    (id1, 'Кызылжарский район'),
    (id1, 'Мамлютский район'),
    (id1, 'район Габита Мусрепова'),
    (id1, 'Тайыншинский район'),
    (id1, 'Тимирязевский район'),
    (id1, 'Уалихановский район'),
    (id1, 'район Шал акына');
END;
$$;
SELECT set_kz ();
-- @block select nodes with current parrent
SELECT node_id,
    parrent_id,
    node_name
FROM node
WHERE parrent_id = 1;
-- @block insert one node
INSERT INTO node (parrent_id, node_name)
VALUES (16, 'dsf')
returning node_id,
    parrent_id,
    node_name;