-- @block select nodes with current parrent
SELECT node_id,
    parrent_id,
    node_name
FROM node
WHERE parrent_id = 16;
##
-- @block insert one node
INSERT INTO node (parrent_id, node_name)
VALUES (14, 'Район 2')
returning node_id,
    parrent_id,
    node_name;
--
-- @block select nodes with current parrent
SELECT n1.node_id,
    n1.parrent_id,
    n1.node_name,
    n1.streets_uuid,
    CASE
        WHEN n.parrent_id >= 0 THEN true
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
    nested;
--
-- @block create street
UPDATE node
SET streets_uuid = COALESCE(streets_uuid, uuid_generate_v4())
WHERE node_id = 64
returning streets_uuid;
--
-- @block insert street
INSERT INTO street (street_uuid, street_name)
VALUES (
        '778d1c3b-cf00-438d-bc31-095f991e2247',
        'Пушкина'
    ),
    (
        '778d1c3b-cf00-438d-bc31-095f991e2247',
        'Жумабаева'
    );
--
-- @block get streets
SELECT street_id,
    street_uuid,
    street_name
FROM street
WHERE street_uuid = '778d1c3b-cf00-438d-bc31-095f991e2247';
--
-- @block create buildinng
INSERT INTO building (street_id, building_name)
VALUES (2, '2'),
    (2, '4'),
    (2, '8');
--
-- @block remove
DELETE FROM node
WHERE node_id = 91
    AND (
        SELECT COUNT(node_name)
        FROM node
        WHERE parrent_id = 91
    ) = 0
returning (
        SELECT COUNT(node_name)
        FROM node
        WHERE parrent_id = 40
    );
--
-- @block
SELECT COALESCE(
        (
            select parrent_id
            from node
            WHERE node_id = 40
        ),
        0
    ) as parrent_id,
    COUNT(n.node_name) as elements_count
FROM node as n
    RIGHT JOIN node ON n.parrent_id = node.node_id
GROUP BY n.parrent_id
HAVING n.parrent_id = 40;
--
-- @block select street with nested
SELECT s.street_id,
    s.street_name,
    s.street_uuid,
    CASE
        WHEN building.street_id >= 0 THEN true
        ELSE false
    END as nested
FROM street as s
    LEFT JOIN building ON building.street_id = s.street_id
WHERE s.street_uuid = '9cf2e6e5-67d3-433f-bcc4-c56f97fbcccb'
GROUP BY s.street_id,
    s.street_uuid,
    s.street_name,
    nested;