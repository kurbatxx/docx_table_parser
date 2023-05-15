SELECT region_id, region_name
FROM region
WHERE country_id = (
        SELECT country_id
        FROM country
        WHERE country_name = 'Казахстан'
    );