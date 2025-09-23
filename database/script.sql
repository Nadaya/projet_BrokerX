CREATE TABLE IF NOT EXISTS public.clients
(
    client_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    name character varying(255) COLLATE pg_catalog."default",
    email character varying(255) COLLATE pg_catalog."default",
    phone character varying(20) COLLATE pg_catalog."default",
    CONSTRAINT clients_pkey PRIMARY KEY (client_id)
);

CREATE TABLE IF NOT EXISTS public.account
(
    account_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    username character varying(50) COLLATE pg_catalog."default" NOT NULL,
    password character varying(255) COLLATE pg_catalog."default" NOT NULL,
    client_id INT NOT NULL REFERENCES clients(client_id) ON DELETE CASCADE,
    portefeuille_id INT NOT NULL REFERENCES portefeuille(id) ON DELETE CASCADE,
    CONSTRAINT accounts_pkey PRIMARY KEY (account_id),
    CONSTRAINT accounts_username_key UNIQUE (username)
);

CREATE TABLE IF NOT EXISTS public.portefeuille
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    balance integer NOT NULL,
    CONSTRAINT portefeuille_pkey PRIMARY KEY (portefeuille_id)
);

CREATE TABLE IF NOT EXISTS public.transactions
(
    transaction_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    portefeuille_id integer NOT NULL,
    amount integer NOT NULL,
    transaction_date timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT transactions_pkey PRIMARY KEY (transaction_id),
    CONSTRAINT fk_portefeuille FOREIGN KEY (portefeuille_id) REFERENCES public.portefeuille (id) ON DELETE CASCADE
);



















-- CREATE TABLE IF NOT EXISTS public.clients
-- (
--     client_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
--     name character varying(255) COLLATE pg_catalog."default",
--     email character varying(255) COLLATE pg_catalog."default",
--     phone character varying(20) COLLATE pg_catalog."default",
--     CONSTRAINT clients_pkey PRIMARY KEY (id)
-- );

-- CREATE TABLE IF NOT EXISTS public.account
-- (
--     account_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
--     username character varying(50) COLLATE pg_catalog."default" NOT NULL,
--     password character varying(255) COLLATE pg_catalog."default" NOT NULL,
--     date_creation TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     client_id INT NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
--     portefeuille_id INT NOT NULL REFERENCES public.portefeuille(portefeuille_id) ON DELETE CASCADE,
--     CONSTRAINT accounts_pkey PRIMARY KEY (account_id),
--     CONSTRAINT accounts_username_key UNIQUE (username)
-- );

-- CREATE TABLE IF NOT EXISTS public.portefeuille
-- (
--     portefeuille_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
--     balance integer NOT NULL,
--     account_id INT UNIQUE NOT NULL,
--     CONSTRAINT portefeuille_pkey PRIMARY KEY (portefeuille_id),
--     FOREIGN KEY (account_id) REFERENCES public.account(account_id) ON DELETE CASCADE
-- );

-- CREATE TABLE IF NOT EXISTS public.transactions
-- (
--     transaction_id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
--     portefeuille_id integer NOT NULL,
--     amount integer NOT NULL,
--     type_transaction ENUM('DEPOT', 'RETRAIT', 'TRANSFERT', 'ACHAT') NOT NULL,
--     transaction_date timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
--     CONSTRAINT transactions_pkey PRIMARY KEY (id),
--     FOREIGN KEY (portefeuille_id) REFERENCES public.portefeuille(portefeuille_id) ON DELETE CASCADE
-- );