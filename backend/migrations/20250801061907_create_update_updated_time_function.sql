CREATE OR REPLACE FUNCTION update_updated_time_column() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_time = CURRENT_TIMESTAMP;
    return NEW;
END;
$$ LANGUAGE 'plpgsql';