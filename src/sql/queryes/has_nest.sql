--EXPLAIN ANALYZE
WITH root AS (
    SELECT node_id,
        node_type,
        parrent_id,
        node_name,
        deputat_uuid
    FROM node
    WHERE parrent_id = $1
)
SELECT root.node_id,
    root.node_type,
    root.parrent_id,
    root.node_name,
    CASE
        WHEN COUNT(node.node_id) > 0 THEN TRUE
        ELSE FALSE
    END AS has_nest,
    root.deputat_uuid
FROM root
    LEFT JOIN node ON node.parrent_id = root.node_id
GROUP BY root.node_id,
    root.node_type,
    root.parrent_id,
    root.node_name,
    root.deputat_uuid
ORDER BY root.node_name