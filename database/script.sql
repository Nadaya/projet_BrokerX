CREATE TABLE IF NOT EXISTS public.clients
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    name character varying(255) COLLATE pg_catalog."default",
    email character varying(255) COLLATE pg_catalog."default",
    phone character varying(20) COLLATE pg_catalog."default",
    CONSTRAINT clients_pkey PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.account
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    username character varying(50) COLLATE pg_catalog."default" NOT NULL,
    password character varying(255) COLLATE pg_catalog."default" NOT NULL,
    role character varying(20) COLLATE pg_catalog."default" NOT NULL,
    client_id INT NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    CONSTRAINT accounts_pkey PRIMARY KEY (id),
    CONSTRAINT accounts_username_key UNIQUE (username)
);

CREATE TABLE IF NOT EXISTS public.portefeuille
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    client_id integer NOT NULL,
    balance integer NOT NULL,
    CONSTRAINT portefeuille_pkey PRIMARY KEY (id),
    CONSTRAINT fk_client FOREIGN KEY (client_id) REFERENCES public.clients (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS public.transactions
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    portefeuille_id integer NOT NULL,
    amount integer NOT NULL,
    transaction_date timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT transactions_pkey PRIMARY KEY (id),
    CONSTRAINT fk_portefeuille FOREIGN KEY (portefeuille_id) REFERENCES public.portefeuille (id) ON DELETE CASCADE
);