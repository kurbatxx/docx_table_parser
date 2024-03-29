-- @block create node table
--CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
--
--DROP COLLATION IF EXISTS numeric_sort;
--CREATE COLLATION numeric_sort (provider = icu, locale = 'ru@colNumeric=yes');
--
DROP TYPE IF EXISTS NodeType;
CREATE TYPE NodeType AS ENUM ('address', 'building', 'street');
--
DROP TABLE IF EXISTS node;
CREATE TABLE node(
    --node_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    node_id SERIAL PRIMARY KEY,
    node_type NodeType DEFAULT 'address',
    parrent_id integer,
    node_name text,
    --deputat_uuid uuid
    deputat_id integer
);
ALTER TABLE node
ADD CONSTRAINT uniq_node_names_on_level UNIQUE (parrent_id, node_name);
--
--ALTER TABLE node
--ALTER COLUMN node_name type TEXT COLLATE numeric_sort;
--
DROP TABLE IF EXISTS deputat_info;
CREATE TABLE deputat_info(
    --deputat_uuid uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    deputat_id SERIAL PRIMARY KEY,
    deputat_name text,
    uch_number integer
);
-- DROP TABLE IF EXISTS street;
-- CREATE TABLE street(
--     street_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
--     street_uuid uuid,
--     street_name text
-- );
-- ALTER TABLE street
-- ADD CONSTRAINT uniq_street_name_with_uuid UNIQUE (street_uuid, street_name);
-- --
-- DROP TABLE IF EXISTS building;
-- CREATE TABLE building(
--     building_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
--     street_id integer,
--     building_name text
-- );
-- ALTER TABLE building
-- ADD CONSTRAINT uniq_building_name_with_street_id UNIQUE (street_id, building_name);
--
INSERT INTO node (parrent_id, node_name)
VALUES (0, 'Казахстан'),
    (0, 'Россия');
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
SELECT node_id into id1
FROM node
WHERE node_name = 'Северо-Казахстанская область';
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