DROP TABLE IF EXISTS country;
CREATE TABLE country(
    country_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    country_name text UNIQUE
);
--
--
INSERT INTO country (country_name)
VALUES ('Казахстан'),
    ('Россия');
--
--
DROP TABLE IF EXISTS region;
CREATE TABLE region(
    region_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    country_id integer,
    region_name text UNIQUE
);
--
--
CREATE OR REPLACE FUNCTION set_kz () RETURNS void language plpgsql as $$
DECLARE id integer;
BEGIN
SELECT country_id into id
FROM country
WHERE country_name = 'Казахстан';
--
INSERT INTO region (country_id, region_name)
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
--
SELECT set_kz ();
DROP FUNCTION IF EXISTS set_country;
--
-- DROP TABLE IF EXISTS rel_country_region;
-- CREATE TABLE rel_country_region(
--     country_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
--     country_name text UNIQUE
-- );
--
--
DROP TABLE IF EXISTS raion_or_city;
CREATE TABLE raion_or_city(
    raion_or_city_id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    region_id integer,
    raion_or_city_name text UNIQUE
);


