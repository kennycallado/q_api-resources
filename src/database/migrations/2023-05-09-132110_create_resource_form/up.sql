CREATE TABLE IF NOT EXISTS resource_form ();

ALTER TABLE resource_form
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN resource_id SERIAL NOT NULL,
  ADD COLUMN question_id SERIAL NOT NULL,
  ADD CONSTRAINT fk_rf_resource_id FOREIGN KEY (resource_id) REFERENCES resources (id) ON DELETE CASCADE;

INSERT INTO resource_form (resource_id, question_id) VALUES
  (5, 2),
  (5, 3),
  (5, 4),
  (5, 5),
  (5, 6)
  ;
