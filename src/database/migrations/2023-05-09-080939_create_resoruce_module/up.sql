-- Modulo es una agrupacion de slides sin questions
CREATE TABLE IF NOT EXISTS resource_module ();

ALTER TABLE resource_module
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN resource_id SERIAL NOT NULL,
  ADD COLUMN slide_id SERIAL NOT NULL,
  ADD CONSTRAINT fk_rm_resource_id FOREIGN KEY (resource_id) REFERENCES resources (id) ON DELETE CASCADE;

INSERT INTO resource_module (resource_id, slide_id) VALUES
  (4, 2),
  (4, 3),
  (4, 4)
  ;
