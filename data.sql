--
-- PostgreSQL database dump
--

-- Dumped from database version 11.1
-- Dumped by pg_dump version 11.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: postgis; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS postgis WITH SCHEMA public;


--
-- Name: EXTENSION postgis; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION postgis IS 'PostGIS geometry, geography, and raster spatial types and functions';


--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: pmfarr
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO pmfarr;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: pmfarr
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO pmfarr;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: pmfarr
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO pmfarr;

--
-- Name: categories; Type: TABLE; Schema: public; Owner: pmfarr
--

CREATE TABLE public.categories (
    title text NOT NULL
);


ALTER TABLE public.categories OWNER TO pmfarr;

--
-- Name: choices; Type: TABLE; Schema: public; Owner: pmfarr
--

CREATE TABLE public.choices (
    id integer NOT NULL,
    question_id integer NOT NULL,
    content text,
    content_type text NOT NULL,
    title text NOT NULL
);


ALTER TABLE public.choices OWNER TO pmfarr;

--
-- Name: choices_id_seq; Type: SEQUENCE; Schema: public; Owner: pmfarr
--

CREATE SEQUENCE public.choices_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.choices_id_seq OWNER TO pmfarr;

--
-- Name: choices_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: pmfarr
--

ALTER SEQUENCE public.choices_id_seq OWNED BY public.choices.id;


--
-- Name: fences; Type: TABLE; Schema: public; Owner: pmfarr
--

CREATE TABLE public.fences (
    title text NOT NULL,
    geo_level integer NOT NULL,
    geo public.geography(MultiPolygon,4326) NOT NULL
);


ALTER TABLE public.fences OWNER TO pmfarr;

--
-- Name: questions; Type: TABLE; Schema: public; Owner: pmfarr
--

CREATE TABLE public.questions (
    id integer NOT NULL,
    survey_id integer NOT NULL,
    question_type text NOT NULL,
    title text NOT NULL
);


ALTER TABLE public.questions OWNER TO pmfarr;

--
-- Name: questions_id_seq; Type: SEQUENCE; Schema: public; Owner: pmfarr
--

CREATE SEQUENCE public.questions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.questions_id_seq OWNER TO pmfarr;

--
-- Name: questions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: pmfarr
--

ALTER SEQUENCE public.questions_id_seq OWNED BY public.questions.id;


--
-- Name: surveys; Type: TABLE; Schema: public; Owner: pmfarr
--

CREATE TABLE public.surveys (
    id integer NOT NULL,
    author text NOT NULL,
    title text NOT NULL,
    description text,
    anonymous boolean DEFAULT true NOT NULL,
    published boolean DEFAULT false NOT NULL,
    date_posted timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    category text NOT NULL
);


ALTER TABLE public.surveys OWNER TO pmfarr;

--
-- Name: surveys_id_seq; Type: SEQUENCE; Schema: public; Owner: pmfarr
--

CREATE SEQUENCE public.surveys_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.surveys_id_seq OWNER TO pmfarr;

--
-- Name: surveys_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: pmfarr
--

ALTER SEQUENCE public.surveys_id_seq OWNED BY public.surveys.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: pmfarr
--

CREATE TABLE public.users (
    username text NOT NULL,
    password text NOT NULL,
    email text NOT NULL,
    first_name text NOT NULL,
    last_name text NOT NULL,
    photo_url text DEFAULT 'https://moonvillageassociation.org/wp-content/uploads/2018/06/default-profile-picture1.jpg'::text,
    is_admin boolean DEFAULT false NOT NULL
);


ALTER TABLE public.users OWNER TO pmfarr;

--
-- Name: votes; Type: TABLE; Schema: public; Owner: pmfarr
--

CREATE TABLE public.votes (
    choice_id integer NOT NULL,
    username text NOT NULL,
    score integer NOT NULL,
    geo public.geography(Point,4326) NOT NULL,
    fence_title text NOT NULL,
    date_voted timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.votes OWNER TO pmfarr;

--
-- Name: users_votes; Type: VIEW; Schema: public; Owner: pmfarr
--

CREATE VIEW public.users_votes AS
 SELECT votes.username,
    questions.survey_id,
    choices.question_id,
    votes.choice_id,
    votes.score
   FROM ((public.votes
     JOIN public.choices ON ((votes.choice_id = choices.id)))
     JOIN public.questions ON ((questions.id = choices.question_id)));


ALTER TABLE public.users_votes OWNER TO pmfarr;

--
-- Name: choices id; Type: DEFAULT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.choices ALTER COLUMN id SET DEFAULT nextval('public.choices_id_seq'::regclass);


--
-- Name: questions id; Type: DEFAULT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.questions ALTER COLUMN id SET DEFAULT nextval('public.questions_id_seq'::regclass);


--
-- Name: surveys id; Type: DEFAULT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.surveys ALTER COLUMN id SET DEFAULT nextval('public.surveys_id_seq'::regclass);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: categories categories_pkey; Type: CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.categories
    ADD CONSTRAINT categories_pkey PRIMARY KEY (title);


--
-- Name: choices choices_pkey; Type: CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.choices
    ADD CONSTRAINT choices_pkey PRIMARY KEY (id);


--
-- Name: fences fences_pkey; Type: CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.fences
    ADD CONSTRAINT fences_pkey PRIMARY KEY (title);


--
-- Name: questions questions_pkey; Type: CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.questions
    ADD CONSTRAINT questions_pkey PRIMARY KEY (id);


--
-- Name: surveys surveys_pkey; Type: CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.surveys
    ADD CONSTRAINT surveys_pkey PRIMARY KEY (id);


--
-- Name: surveys surveys_title_key; Type: CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.surveys
    ADD CONSTRAINT surveys_title_key UNIQUE (title);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (username);


--
-- Name: votes votes_pkey; Type: CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.votes
    ADD CONSTRAINT votes_pkey PRIMARY KEY (choice_id, username);


--
-- Name: choices choices_question_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.choices
    ADD CONSTRAINT choices_question_id_fkey FOREIGN KEY (question_id) REFERENCES public.questions(id) ON DELETE CASCADE;


--
-- Name: questions questions_survey_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.questions
    ADD CONSTRAINT questions_survey_id_fkey FOREIGN KEY (survey_id) REFERENCES public.surveys(id) ON DELETE CASCADE;


--
-- Name: surveys surveys_author_fkey; Type: FK CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.surveys
    ADD CONSTRAINT surveys_author_fkey FOREIGN KEY (author) REFERENCES public.users(username) ON DELETE CASCADE;


--
-- Name: surveys surveys_category_fkey; Type: FK CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.surveys
    ADD CONSTRAINT surveys_category_fkey FOREIGN KEY (category) REFERENCES public.categories(title) ON DELETE CASCADE;


--
-- Name: votes votes_choice_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.votes
    ADD CONSTRAINT votes_choice_id_fkey FOREIGN KEY (choice_id) REFERENCES public.choices(id) ON DELETE CASCADE;


--
-- Name: votes votes_fence_title_fkey; Type: FK CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.votes
    ADD CONSTRAINT votes_fence_title_fkey FOREIGN KEY (fence_title) REFERENCES public.fences(title) ON DELETE CASCADE;


--
-- Name: votes votes_username_fkey; Type: FK CONSTRAINT; Schema: public; Owner: pmfarr
--

ALTER TABLE ONLY public.votes
    ADD CONSTRAINT votes_username_fkey FOREIGN KEY (username) REFERENCES public.users(username) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

