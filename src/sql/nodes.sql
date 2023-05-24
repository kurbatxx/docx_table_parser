-- @block create node table
DROP TABLE IF EXISTS node;
CREATE TABLE node(
    node_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    parrent_id integer,
    node_name text,
    streets_uuid uuid
);
ALTER TABLE node
ADD CONSTRAINT uniq_node_names_on_level UNIQUE (parrent_id, node_name);
--
--
DROP TABLE IF EXISTS street;
CREATE TABLE street(
    street_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    street_uuid uuid,
    street_name text
);

ALTER TABLE street
ADD CONSTRAINT uniq_street_name_with_uuid UNIQUE (street_uuid, street_name);

--
--

DROP TABLE IF EXISTS building;
CREATE TABLE building(
    building_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    street_id integer,
    building_name text
);

ALTER TABLE building
ADD CONSTRAINT uniq_building_name_with_street_id UNIQUE (street_id, building_name);

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
WHERE parrent_id = 16;
-- @block insert one node
INSERT INTO node (parrent_id, node_name)
VALUES (14, 'Район 2')
returning node_id,
    parrent_id,
    node_name;
-- @block select nodes with current parrent
SELECT n1.node_id,
    n1.parrent_id,
    n1.node_name,
    n1.streets_uuid,
    CASE
        WHEN n.parrent_id > 0 THEN true
        ELSE false
    END as nested
FROM node as n
    RIGHT JOIN (
        SELECT node_id,
            parrent_id,
            node_name,
            streets_uuid
        FROM node
        WHERE parrent_id = 0
    ) as n1 ON n1.node_id = n.parrent_id
GROUP BY n1.node_id,
    n1.node_name,
    n1.parrent_id,
    n1.streets_uuid,
    nested 
    
-- @block create street
UPDATE node
SET streets_uuid = COALESCE(streets_uuid, uuid_generate_v4())
WHERE node_id = 19
returning streets_uuid 

-- @block insert street
INSERT INTO street (street_uuid, street_name)
VALUES 
    (
        '778d1c3b-cf00-438d-bc31-095f991e2247',
        'Пушкина'
    ),
    (
        '778d1c3b-cf00-438d-bc31-095f991e2247',
        'Жумабаева'
    )

-- @block get streets

SELECT street_id, street_uuid, street_name
FROM street
WHERE street_uuid = '778d1c3b-cf00-438d-bc31-095f991e2247'

-- @block create t
DROP TABLE IF EXISTS building;
CREATE TABLE building(
    building_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    street_id integer,
    building_name text
);

ALTER TABLE building
ADD CONSTRAINT uniq_building_name_with_street_id UNIQUE (street_id, building_name);