-- CREATE DATABASE IF NOT EXISTS nginxdb
-- 
-- CREATE TABLE IF NOT EXISTS  nginxdb.access_logs (
--     message String
-- )
--     ENGINE = MergeTree()
--     ORDER BY tuple()

-- -- Create database for logs
-- CREATE DATABASE IF NOT EXISTS logs;
-- 
-- -- Create table for application logs
-- CREATE TABLE IF NOT EXISTS logs.logs
-- (
--     timestamp DateTime64(9) DEFAULT now(),
--     trace_id String,
--     span_id String,
--     severity_text String,
--     severity_number UInt8,
--     service_name String,
--     service_version String,
--     body String,
--     resource_attributes Map(String, String),
--     attributes Map(String, String)
--     )
--     ENGINE = MergeTree()
--     ORDER BY (timestamp, service_name)
--     TTL timestamp + INTERVAL 30 DAY;
-- 
-- -- Create materialized view for quick access to error logs
-- CREATE MATERIALIZED VIEW IF NOT EXISTS logs.error_logs
-- ENGINE = MergeTree()
-- ORDER BY (timestamp, service_name)
-- TTL timestamp + INTERVAL 30 DAY
-- AS SELECT *
--    FROM logs.logs
--    WHERE severity_text IN ('ERROR', 'FATAL');

    -- -- old_1
-- -- Create database for logs
-- CREATE DATABASE IF NOT EXISTS logs;
-- 
-- -- Create table for application logs
-- CREATE TABLE IF NOT EXISTS logs.logs
-- (
--     timestamp DateTime64(9) DEFAULT now(),
--     trace_id String,
--     span_id String,
--     severity_text String,
--     severity_number UInt8,
--     service_name String,
--     service_version String,
--     body String,
--     resource_attributes Map(String, String),
--     attributes Map(String, String)
--     )
--     ENGINE = MergeTree()
--     ORDER BY (timestamp, service_name)
--     TTL toDateTime(timestamp) + INTERVAL 30 DAY;

   

-- -- Create database for logs
-- CREATE DATABASE IF NOT EXISTS logs;
-- DROP TABLE IF EXISTS logs.logs;
-- 
-- -- Create table for application logs
-- CREATE TABLE IF NOT EXISTS logs.logs
-- (
--     timestamp DateTime64(9) DEFAULT now(),  -- More precise timestamps
--     observed_timestamp DateTime64(9),       -- Observed timestamp from Vector logs
--     trace_id String DEFAULT '',             -- Trace ID for distributed tracing
--     span_id String DEFAULT '',              -- Span ID for distributed tracing
--     severity_text String,                   -- Severity level (e.g., "Info", "Error")
--     severity_number UInt8,                   -- Numeric severity level
--     service_name String,                     -- Name of the service
--     service_version String DEFAULT '',       -- Version of the service
--     body String DEFAULT '',                  -- The log message/body
--     source_type String DEFAULT '',           -- (Optional) Log source type (e.g., "opentelemetry")
--     flags UInt8 DEFAULT 0,                   -- Flags from OpenTelemetry
--     resource_attributes String,                -- String column for resource attributes
--     attributes String                          -- String column for log attributes
--     )
--     ENGINE = MergeTree()
--     ORDER BY (timestamp, service_name)
--     TTL toDateTime(timestamp) + INTERVAL 30 DAY;

-- -- Additional Table for Traces (Optional)
-- CREATE TABLE IF NOT EXISTS logs.traces
-- (
--     timestamp DateTime64(9) DEFAULT now(),
--     trace_id String,
--     span_id String,
--     parent_span_id String DEFAULT '',
--     service_name String,
--     service_version String DEFAULT '',
--     span_kind String DEFAULT '',
--     status_code UInt8,
--     status_message String DEFAULT '',
--     duration UInt64,
--     attributes String
--     )
--     ENGINE = MergeTree()
--     ORDER BY (timestamp, service_name)
--     TTL toDateTime(timestamp) + INTERVAL 30 DAY;


-- -- -- New Gemini
-- -- Create database for logs
-- CREATE DATABASE IF NOT EXISTS logs;
-- DROP TABLE IF EXISTS logs.logs;
-- -- Create table for application logs
-- CREATE TABLE IF NOT EXISTS logs.logs(
--     timestamp DateTime64(9),
--     observed_timestamp DateTime64(9),
--     trace_id FixedString(16),
--     span_id FixedString(8),
--     severity_number UInt8,
--     severity_text LowCardinality(String),
--     message String,
--     attributes Nested (
-- --         Time String
--         Time Nullable(String)
--     ),
--     resources Nested (
--         service_instance_id UUID,
--         service_name LowCardinality(String),
--         service_version LowCardinality(String),
--         telemetry_sdk_language LowCardinality(String),
--         telemetry_sdk_name LowCardinality(String),
--         telemetry_sdk_version LowCardinality(String)
--     ),
--     scope Nested (
--         name LowCardinality(String)
--     ),
--     dropped_attributes_count UInt32,
--     flags UInt32,
--     source_type LowCardinality(String)
--     ) ENGINE = MergeTree() PARTITION BY toYYYYMM(timestamp)
--     ORDER BY
--     (
--         timestamp, severity_number, trace_id
--     );

-- Create database for logs
CREATE DATABASE IF NOT EXISTS logs;
DROP TABLE IF EXISTS logs.logs;
-- Create table for application logs
CREATE TABLE IF NOT EXISTS logs.logs(
    timestamp DateTime64(9) DEFAULT now(),
    observed_timestamp DateTime64(9),
    trace_id FixedString(32),
    span_id FixedString(16),
    message Nullable(String),
    severity_number UInt8
    ) ENGINE = MergeTree() PARTITION BY toYYYYMM(timestamp)
    ORDER BY
    (
        timestamp
    );