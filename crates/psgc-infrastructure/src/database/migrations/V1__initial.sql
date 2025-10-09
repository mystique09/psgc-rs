-- migrations/v1_initial.sql
-- Create regions table
CREATE TABLE
    regions (
        id UUID PRIMARY KEY,
        code VARCHAR(255) NOT NULL,
        correspondence_code VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        designation VARCHAR(255) NOT NULL,
        population BIGINT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

-- Create provinces table
CREATE TABLE
    provinces (
        id UUID PRIMARY KEY,
        code VARCHAR(255) NOT NULL,
        correspondence_code VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        population BIGINT NOT NULL,
        income_class VARCHAR(255) NOT NULL,
        region_id UUID REFERENCES regions (id),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

-- Create districts table
CREATE TABLE
    districts (
        id UUID PRIMARY KEY,
        code VARCHAR(255) NOT NULL,
        correspondence_code VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        population BIGINT NOT NULL,
        region_id UUID REFERENCES regions (id),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

-- Create cities table
CREATE TABLE
    cities (
        id UUID PRIMARY KEY,
        code VARCHAR(255) NOT NULL,
        correspondence_code VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        population BIGINT NOT NULL,
        city_class VARCHAR(255) NOT NULL,
        income_class VARCHAR(255) NOT NULL,
        region_id UUID REFERENCES regions (id),
        province_id UUID REFERENCES provinces (id),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

-- Create sub_municipalities table
CREATE TABLE
    sub_municipalities (
        id UUID PRIMARY KEY,
        code VARCHAR(255) NOT NULL,
        correspondence_code VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        population BIGINT NOT NULL,
        city_id UUID NOT NULL REFERENCES cities (id),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

-- Create municipalities table
CREATE TABLE
    municipalities (
        id UUID PRIMARY KEY,
        code VARCHAR(255) NOT NULL,
        correspondence_code VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        population BIGINT NOT NULL,
        income_class VARCHAR(255) NOT NULL,
        region_id UUID REFERENCES regions (id),
        province_id UUID REFERENCES provinces (id),
        city_id UUID REFERENCES cities (id),
        district_id UUID REFERENCES districts (id),
        sub_municipality_id UUID REFERENCES sub_municipalities (id),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

-- Create barangays table
CREATE TABLE
    barangays (
        id UUID PRIMARY KEY,
        code VARCHAR(255) NOT NULL,
        correspondence_code VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        population BIGINT NOT NULL,
        urban_rural VARCHAR(255) NOT NULL,
        city_id UUID REFERENCES cities (id),
        municipality_id UUID REFERENCES municipalities (id),
        district_id UUID REFERENCES districts (id),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );