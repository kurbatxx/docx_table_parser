--EXPLAIN ANALYZE

WITH root AS (SELECT node_id, parrent_id, node_name, streets_uuid 
FROM node 
WHERE parrent_id = 0 )

SELECT root.node_id, root.parrent_id, root.node_name, 
CASE 
	WHEN COUNT(node.node_id) > 0 THEN TRUE
	ELSE FALSE
END
AS has_nest, root.streets_uuid FROM root

LEFT JOIN node 
ON node.parrent_id = root.node_id
GROUP BY root.node_id, root.parrent_id, root.node_name, root.streets_uuid
ORDER BY root.node_name
