CREATE TABLE IF NOT EXISTS resources ();

ALTER TABLE resources
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN resource_type VARCHAR NOT NULL,
  ADD COLUMN title VARCHAR NOT NULL,
  ADD COLUMN description VARCHAR NOT NULL,
  ADD COLUMN created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  ADD COLUMN updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;

ALTER TABLE resources ADD CONSTRAINT resources_resource_type_check CHECK (resource_type IN ('slides', 'module', 'form', 'external'));

SELECT diesel_manage_updated_at('resources');
INSERT INTO resources (resource_type, title, description) VALUES
  ('slides', 'Título 1', 'Descripción del título 1'),
  ('slides', 'Título 2', 'Descripción del título 2'),
  ('slides', 'Título 3', 'Descripción del título 3')
  ;

-- Idea de copilot
-- ALTER TABLE resources ADD CONSTRAINT resources_resource_type_check CHECK (resource_type IN ('slides', 'forms', 'externals', 'articles'));

-- ALTER TABLE resources REPLICA IDENTITY FULL;
-- CREATE PUBLICATION resources_pub FOR TABLE resources;
