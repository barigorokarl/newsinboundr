-- Your SQL goes here
create table source_atom (
	id SERIAL PRIMARY KEY,
	url text,
	display_name varchar(200),
	color varchar(10)
);

create index source_atom_display_name on source_atom (display_name);
