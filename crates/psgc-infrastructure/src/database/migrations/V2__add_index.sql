-- Regions table indexes
CREATE INDEX idx_region_code ON regions(code);
CREATE INDEX idx_region_correspondence_code ON regions(correspondence_code);
CREATE INDEX idx_region_name ON regions(name);

-- Provinces table indexes
CREATE INDEX idx_province_code ON provinces(code);
CREATE INDEX idx_province_correspondence_code ON provinces(correspondence_code);
CREATE INDEX idx_province_name ON provinces(name);
CREATE INDEX idx_province_region_id ON provinces(region_id);
CREATE INDEX idx_province_income_class ON provinces(income_class);

-- Districts table indexes
CREATE INDEX idx_district_code ON districts(code);
CREATE INDEX idx_district_correspondence_code ON districts(correspondence_code);
CREATE INDEX idx_district_name ON districts(name);
CREATE INDEX idx_district_region_id ON districts(region_id);

-- Cities table indexes
CREATE INDEX idx_city_code ON cities(code);
CREATE INDEX idx_city_correspondence_code ON cities(correspondence_code);
CREATE INDEX idx_city_name ON cities(name);
CREATE INDEX idx_city_region_id ON cities(region_id);
CREATE INDEX idx_city_province_id ON cities(province_id);
CREATE INDEX idx_city_city_class ON cities(city_class);
CREATE INDEX idx_city_income_class ON cities(income_class);

-- Municipalities table indexes
CREATE INDEX idx_municipality_code ON municipalities(code);
CREATE INDEX idx_municipality_correspondence_code ON municipalities(correspondence_code);
CREATE INDEX idx_municipality_name ON municipalities(name);
CREATE INDEX idx_municipality_region_id ON municipalities(region_id);
CREATE INDEX idx_municipality_province_id ON municipalities(province_id);
CREATE INDEX idx_municipality_district_id ON municipalities(district_id);
CREATE INDEX idx_municipality_parent_id ON municipalities(parent_municipality_id);
CREATE INDEX idx_municipality_income_class ON municipalities(income_class);

-- Barangays table indexes
CREATE INDEX idx_barangay_code ON barangays(code);
CREATE INDEX idx_barangay_correspondence_code ON barangays(correspondence_code);
CREATE INDEX idx_barangay_name ON barangays(name);
CREATE INDEX idx_barangay_city_id ON barangays(city_id);
CREATE INDEX idx_barangay_municipality_id ON barangays(municipality_id);
CREATE INDEX idx_barangay_district_id ON barangays(district_id);
CREATE INDEX idx_barangay_urban_rural ON barangays(urban_rural);

-- Composite indexes for common relationship queries
CREATE INDEX idx_province_region_code ON provinces(region_id, code);
CREATE INDEX idx_city_province_region ON cities(province_id, region_id);
CREATE INDEX idx_municipality_province_region ON municipalities(province_id, region_id);
CREATE INDEX idx_barangay_municipality_city ON barangays(municipality_id, city_id);