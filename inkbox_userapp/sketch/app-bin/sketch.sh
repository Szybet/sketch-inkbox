#!/system-bin/sh
cd /app-bin

VERSION=$(cat /mnt/onboard/.kobo/version)

PRODUCT_CONVERSION=none

# Nia
if [ "$VERSION" = "N306" ]; then
PRODUCT_CONVERSION=luna

# Mini
elif [ "$VERSION" = "N705" ]; then
PRODUCT_CONVERSION=pixie

# Touch B
elif [ "$VERSION" = "N905B" ]; then
PRODUCT_CONVERSION=pixie

# Touch C
elif [ "$VERSION" = "N905C" ]; then
PRODUCT_CONVERSION=pixie

# Glo
elif [ "$VERSION" = "N613" ]; then
PRODUCT_CONVERSION=kraken

# Glo hd
elif [ "$VERSION" = "N437" ]; then
PRODUCT_CONVERSION=pixie

# Kobo Aura (2nd Edition)
elif [ "$VERSION" = "N236" ]; then
# Idk, its propably the AuraH2O, if not correct this
PRODUCT_CONVERSION=dahlia

fi

echo $PRODUCT_CONVERSION

env PRODUCT="$PRODUCT_CONVERSION" PATH="/app-bin:/system-bin" LD_LIBRARY_PATH="/system-lib/lib:/system-lib/qt/lib:/app-lib" /system-lib/lib/ld-linux-armhf.so.3 ./sketch.bin /app-data
