cd /Users/greg/dev/icu4x

cargo run --bin icu4x-datagen -- \
   --cldr-tag 39.0.0 \
   --all-keys \
   --all-locales \
   --syntax bincode \
   --out /Users/greg/lem/spec-icu/data \
   -v
