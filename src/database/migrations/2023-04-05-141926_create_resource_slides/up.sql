CREATE TABLE IF NOT EXISTS resource_slides ();

ALTER TABLE resource_slides
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN resource_id SERIAL NOT NULL,
  ADD COLUMN slide_id SERIAL NOT NULL,
  ADD CONSTRAINT fk_rs_resource_id FOREIGN KEY (resource_id) REFERENCES resources (id) ON DELETE CASCADE;

INSERT INTO resource_slides (resource_id, slide_id) VALUES
  (1, 1),
  (1, 2),
  (1, 3),
  (2, 4),
  (2, 5),
  (2, 6),
  (3, 7),
  (3, 8)
  ;
