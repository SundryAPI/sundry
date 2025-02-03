INSERT INTO public.organizations (name) VALUES ('Personal');

INSERT INTO public.application_api_keys (organization_id, key_id, key_hash) VALUES (1, '5nSVZr6pn4TJ6av8BbVOl2XXmCGDqz2D', '$argon2id$v=19$m=19456,t=2,p=1$/grVryTYVCIqr3Ol57SXwg$qa8QXF/q/Q6O+zd84QyIDsVsCx3u2jdyvaDDDo5xgxs');

INSERT INTO public.users (email) VALUES 
  ('dev1@test.com'), 
  ('dev2@test.com'), 
  ('dev3@test.com');

INSERT INTO public.user_organizations (user_id, organization_id, role) VALUES 
  (1, 1, 'owner'),
  (2, 1, 'owner'),
  (3, 1, 'owner');

INSERT INTO public.user_api_keys (user_id, organization_id, description, key_id, key_hash) VALUES 
  (1, 1, 'filler', 'ArWSOTAe6JShzlKbf5KNND6vTi8WpGQe', '$argon2id$v=19$m=19456,t=2,p=1$7OF0IZJd1w/9jTLdxDtmEw$7E1izE39cOfRGzTmonrOQz1QcC0WVwvI60BHPzqZpKs'),
  (2, 1, 'filler', 'Hn9kCs29WotIaTfWJWo-aaSMwNu0dPfV', '$argon2id$v=19$m=19456,t=2,p=1$Zo4XjGSS4DKMORi63HNPOg$e7ekEKn4ficLuF0pMiNZuSLhGI3fA/499peK4aPY/mM'),
  (3, 1, 'filler', '7KPkflaTjZ+CsXHhEu4ge-cTt4iOY9Cl', '$argon2id$v=19$m=19456,t=2,p=1$QXDAEgjdMhF8E/XqSMZt0Q$ewG/sWg3owyAirY/ewfmsKTxPhEMt7VBzNen8nnJdBg');

-- Maybe this should be in the github fixture?
INSERT INTO user_oauth_accounts (user_id, provider_id, provider_data) VALUES
  (1, 1, '{"login": "dev1"}'),
  (2, 1, '{"login": "dev2"}'),
  (3, 1, '{"login": "dev3"}');
