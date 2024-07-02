psql postgresql://syntaxmakers:syntaxmakers@localhost:5433/syntaxmakers -f ./tools/truncatedb.sql

psql postgresql://syntaxmakers:syntaxmakers@localhost:5433/syntaxmakers -f ./tools/setup-dev-data.sql

cargo test -- --nocapture